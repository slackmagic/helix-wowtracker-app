use helix_wowtracker_domain::storage::error::*;
use helix_wowtracker_domain::storage::traits::LevelXpStorageTrait;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;

pub struct MemoryIndexStorage {
    pub index: BTreeMap<String, u32>,
}

impl MemoryIndexStorage {
    pub fn new(path: String) -> StorageResult<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let index: BTreeMap<String, u32> = serde_json::from_reader(reader)?;

        Ok(MemoryIndexStorage { index: index })
    }
}

impl LevelXpStorageTrait for MemoryIndexStorage {
    fn get_index(&self, value: String) -> StorageResult<Option<u32>> {
        match self.index.get(&value) {
            Some(result) => Ok(Some(*result)),
            None => Ok(None),
        }
    }
}

impl MemoryIndexStorage {
    pub fn load_index(&self) -> StorageResult<()> {
        Ok(())
    }
}
