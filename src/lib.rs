use std::{io, path::Path};

pub struct ActionKV {}

impl ActionKV {
    pub fn open(fname: &Path) -> io::Result<ActionKV> {
        Ok(ActionKV {})
    }

    pub fn load(&mut self) -> io::Result<()> {
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
}
