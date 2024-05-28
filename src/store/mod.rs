#[cfg(target_arch = "wasm32")]
mod local_storage;

#[cfg(target_arch = "wasm32")]
pub use local_storage::Store;

#[cfg(not(target_arch = "wasm32"))]
mod rocks_db;

#[cfg(not(target_arch = "wasm32"))]
pub use rocks_db::Store;

#[cfg(target_os = "android")]
use std::path::Path;

use serde::{de::DeserializeOwned, Serialize};

pub enum Location<'a> {
    #[cfg(target_arch = "wasm32")]
    LocalStorage(&'a str),
    #[cfg(not(target_arch = "wasm32"))]
    CustomPath(&'a std::path::Path),
}

pub trait StoreImpl {
    type GetError;
    type SetError;

    fn get<T: DeserializeOwned>(&self, key: &str) -> Result<T, Self::GetError>;
    fn set<T: Serialize>(&mut self, key: &str, value: &T) -> Result<(), Self::SetError>;
    fn clear(&mut self) -> Result<(), Self::SetError>;
}

#[cfg(target_arch = "wasm32")]
pub fn new_store(identifier: &str) -> Store {
    Store::new(Location::LocalStorage(identifier))
}

#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
fn get_desktop_path(identifier: &str) -> std::path::PathBuf {
    let [qualifier, organization, application] = identifier.split('.').collect::<Vec<&str>>()[..3] else {
        panic!("Invalid identifier");
    };

    directories::ProjectDirs::from(qualifier, organization, application)
        .expect("No local storage")
        .data_dir()
        .to_path_buf()
}

#[cfg(target_os = "android")]
fn get_android_path(identifier: &str) -> std::path::PathBuf {
    let path_str = &["/data/data/", identifier,"/db"].concat();
    Path::new(path_str).to_path_buf()
}


#[cfg(not(target_arch = "wasm32"))]
pub fn new_store(identifier: &str) -> Store {
    #[cfg(not(target_os = "android"))]
    let path = get_desktop_path(identifier);
    
    #[cfg(target_os = "android")]
    let path = get_android_path(identifier);

    Store::new(Location::CustomPath(path.as_ref()))
}
