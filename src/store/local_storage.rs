use crate::store::{Location, StoreImpl};
use web_sys::wasm_bindgen;

impl<'a> Location<'a> {
    pub fn get_path(&self) -> &str {
        match self {
            Self::LocalStorage(path) => path,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct LocalStorage {
    prefix: String,
}

pub use LocalStorage as Store;

#[derive(thiserror::Error, Debug)]
pub enum GetError {
    #[error("No value found for the given key")]
    NotFound,
    #[error("Error deserializing json")]
    Json(#[from] serde_json::Error),
    #[error("JavaScript error from getItem")]
    GetItem(wasm_bindgen::JsValue),
}

#[derive(thiserror::Error, Debug)]
pub enum SetError {
    #[error("JavaScript error from setItem")]
    SetItem(wasm_bindgen::JsValue),
    #[error("Error serializing as json")]
    Json(#[from] serde_json::Error),
    #[error("JavaScript error from clear")]
    Clear(wasm_bindgen::JsValue),
}

impl LocalStorage {
    fn storage(&self) -> web_sys::Storage {
        web_sys::window()
            .expect("No window")
            .local_storage()
            .expect("Failed to get local storage")
            .expect("No local storage")
    }

    pub fn new(location: Location) -> Self {
        let path = location.get_path();
        Self {
            prefix: path.to_string(),
        }
    }

    fn format_key(&self, key: &str) -> String {
        format!("{}{}", self.prefix, key)
    }
}

impl StoreImpl for LocalStorage {
    type GetError = GetError;
    type SetError = SetError;

    fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<T, GetError> {
        let storage = self.storage();
        let key = self.format_key(key);
        let entry = storage.get_item(&key).map_err(GetError::GetItem)?;
        let json = entry.as_ref().ok_or(GetError::NotFound)?;
        let value: T = serde_json::from_str(json)?;
        Ok(value)
    }

    fn set<T: serde::Serialize>(&mut self, key: &str, value: &T) -> Result<(), SetError> {
        let json = serde_json::to_string(value)?;
        let storage = self.storage();
        let key = self.format_key(key);
        storage.set_item(&key, &json).map_err(SetError::SetItem)?;
        Ok(())
    }

    fn clear(&mut self) -> Result<(), SetError> {
        let storage = self.storage();
        let length = storage.length().map_err(SetError::Clear)?;
        let prefix = &self.prefix;
        for index in 0..length {
            if let Some(key) = storage.key(index).map_err(SetError::Clear)? {
                if key.starts_with(prefix) {
                    storage.delete(&key).map_err(SetError::Clear)?;
                }
            }
        }
        Ok(())
    }
}
