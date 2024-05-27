#[cfg(target_arch = "wasm32")]
mod local_storage;

#[cfg(target_arch = "wasm32")]
pub use local_storage::Store;

#[cfg(not(target_arch = "wasm32"))]
mod rocks_db;

#[cfg(not(target_arch = "wasm32"))]
pub use rocks_db::Store;

use serde::{de::DeserializeOwned, Serialize};

pub enum Location<'a> {
    #[cfg(target_arch = "wasm32")]
    PlatformDefault(&'a PlatformDefault),
    #[cfg(not(target_arch = "wasm32"))]
    CustomPath(&'a std::path::Path),
}

pub trait StoreImpl {
    type GetError;
    type SetError;

    fn set_string(&mut self, key: &str, value: &str) -> Result<(), Self::SetError> {
        self.set(key, &value.to_string())
    }
    fn get<T: DeserializeOwned>(&self, key: &str) -> Result<T, Self::GetError>;
    fn set<T: Serialize>(&mut self, key: &str, value: &T) -> Result<(), Self::SetError>;
    fn clear(&mut self) -> Result<(), Self::SetError>;
}

#[cfg(target_arch = "wasm32")]
pub struct PlatformDefault {
    qualifier: String,
    organization: String,
    application: String,
}

#[cfg(target_arch = "wasm32")]
pub fn new_store(qualifier: &str, organization: &str, application: &str) -> Store {
    let config = PlatformDefault {
        qualifier: qualifier.to_string(),
        organization: organization.to_string(),
        application: application.to_string(),
    };
    Store::new(Location::PlatformDefault(&config))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn new_store(qualifier: &str, organization: &str, application: &str) -> Store {
    let binding = directories::ProjectDirs::from(qualifier, organization, application)
        .expect("No local storage");
    let path = binding.data_dir();
    Store::new(Location::CustomPath(path.as_ref()))
}
