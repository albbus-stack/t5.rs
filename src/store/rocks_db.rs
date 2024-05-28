use crate::store::{Location, StoreImpl};
use serde::{de::DeserializeOwned, Serialize};

impl<'a> Location<'a> {
    pub fn get_path(&self) -> std::path::PathBuf {
        match self {
            Self::CustomPath(path) => path.to_path_buf(),
        }
    }
}

#[derive(Debug)]
pub struct RocksDB {
    db: rocksdb::DB,
}

pub use RocksDB as Store;

#[derive(thiserror::Error, Debug)]
pub enum GetError {
    #[error("RocksDB error")]
    Rocksdb(#[from] rocksdb::Error),
    #[error("MessagePack deserialization error")]
    MessagePack(#[from] rmp_serde::decode::Error),
    #[error("No value found for the given key")]
    NotFound,
}

#[derive(thiserror::Error, Debug)]
pub enum SetError {
    #[error("RocksDB error")]
    Rocksdb(#[from] rocksdb::Error),
    #[error("MessagePack serialization error")]
    MessagePack(#[from] rmp_serde::encode::Error),
}

impl RocksDB {
    pub fn new(location: Location) -> Self {
        let mut options = rocksdb::Options::default();
        options.set_error_if_exists(false);
        options.create_if_missing(true);
        options.create_missing_column_families(true);

        let db_path = location.get_path().join("rocks_db_store");
        let db = rocksdb::DB::open(&options, db_path).expect("Failed to init key value store");
        Self { db }
    }
}

impl StoreImpl for RocksDB {
    type GetError = GetError;
    type SetError = SetError;

    fn set<T: Serialize>(&mut self, key: &str, value: &T) -> Result<(), Self::SetError> {
        let mut serializer = rmp_serde::Serializer::new(Vec::new()).with_struct_map();
        value.serialize(&mut serializer)?;
        self.db.put(key, serializer.into_inner())?;

        Ok(())
    }

    fn get<T: DeserializeOwned>(&self, key: &str) -> Result<T, Self::GetError> {
        let bytes = self.db.get(key)?.ok_or(Self::GetError::NotFound)?;
        let value = rmp_serde::from_slice(&bytes)?;
        Ok(value)
    }

    fn clear(&mut self) -> Result<(), Self::SetError> {
        let kv_iter = self.db.iterator(rocksdb::IteratorMode::Start);

        for kv in kv_iter {
            let (key, _) = kv?;
            self.db.delete(key)?;
        }

        Ok(())
    }
}
