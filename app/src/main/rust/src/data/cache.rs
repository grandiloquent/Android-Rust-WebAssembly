use ndk::asset::AssetManager;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use crate::utils::asset::read_resource_file;

pub struct Cache {
    ass: AssetManager,
    data: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl Cache {
    pub fn new(ass: AssetManager) -> Cache {
        Cache {
            ass,
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        match self.data.write() {
            Ok(mut v) => {
                match v.get(key) {
                    None => {
                        match read_resource_file(&self.ass, key) {
                            Ok(value) => {
                                v.insert(key.to_string(), value.clone());
                                Some(value)
                            }
                            Err(err) => {
                                log::error!("{}",err.to_string());
                                None
                            }
                        }
                    }
                    Some(v) => {
                        Some((*v).clone())
                    }
                }
            }
            Err(_err) => {
                None
            }
        }
    }
}