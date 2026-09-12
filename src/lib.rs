use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, BufReader, Read, Seek, SeekFrom},
    path::Path,
};

use byteorder::{LittleEndian, ReadBytesExt};
use crc::{self, CRC_32_ISO_HDLC, Crc};
use serde::{Deserialize, Serialize};

type ByteString = Vec<u8>;
type ByteStr = [u8];
const CRC32_IEEE: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

#[derive(Debug, Serialize, Deserialize)]
struct KeyValuePair {
    pub key: ByteString,
    pub value: ByteString,
}

pub struct ActionKV {
    f: File,
    pub index: HashMap<ByteString, u64>,
}

impl ActionKV {
    pub fn open(fname: &Path) -> io::Result<Self> {
        let f = OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(fname)?;

        Ok(Self {
            f,
            index: HashMap::new(),
        })
    }

    pub fn load(&mut self) -> io::Result<()> {
        let mut f = BufReader::new(&mut self.f);

        loop {
            let current_position = f.seek(SeekFrom::Current(0))?;
            let maybe_kv = Self::process_record(&mut f);
            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => match err.kind() {
                    io::ErrorKind::UnexpectedEof => {
                        break;
                    }
                    _ => return Err(err),
                },
            };
            self.index.insert(kv.key, current_position);
        }
        Ok(())
    }

    pub fn get(&self, key: &str) -> io::Result<Option<()>> {
        Ok(Some(()))
    }

    pub fn delete(&self, key: &str) -> io::Result<()> {
        Ok(())
    }

    pub fn insert(&self, key: &str, value: &str) -> io::Result<()> {
        Ok(())
    }

    pub fn update(&self, key: &str, value: &str) -> io::Result<()> {
        Ok(())
    }

    pub fn process_record<R: Read>(f: &mut R) -> io::Result<KeyValuePair> {
        let saved_checksum = f.read_u32::<LittleEndian>()?;
        let key_len = f.read_u32::<LittleEndian>()?;
        let val_len = f.read_u32::<LittleEndian>()?;
        let data_len = key_len + val_len;

        let mut data = ByteString::with_capacity(data_len as usize);
        {
            f.by_ref().take(data_len as u64).read_to_end(&mut data)?;
        }

        debug_assert_eq!(data.len(), data_len as usize);

        let checksum = CRC32_IEEE.checksum(&data);

        if checksum != saved_checksum {
            panic!(
                "data corruption encountered ({:08x} != {:08x})",
                checksum, saved_checksum
            );
        }

        let value = data.split_off(key_len as usize);
        let key = data;

        Ok(KeyValuePair { key, value })
    }
}
