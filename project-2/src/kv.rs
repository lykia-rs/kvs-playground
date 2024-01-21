use crate::error::Result;
use std::path::Path;
pub struct KvStore;

impl KvStore {
    pub fn new() -> KvStore {
        KvStore
    }

    pub fn open(path: &Path) -> Result<KvStore> {
        Ok(KvStore::new())
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(Some("aaaa".into()))
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        Ok(())
    }
}
