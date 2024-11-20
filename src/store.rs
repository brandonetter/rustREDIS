use crate::types::{RedisGetResult, RedisValue};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

pub struct RedisStore {
    data: Arc<Mutex<HashMap<String, RedisValue>>>,
}

impl RedisStore {
    pub fn new() -> Self {
        RedisStore {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn set(
        &self,
        key: String,
        value: String,
        px: Option<u64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let expires_at = px.map(|millis| SystemTime::now() + Duration::from_millis(millis));
        let value = RedisValue {
            data: value,
            expires_at,
        };
        let mut store = self.data.lock().unwrap();
        store.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: &str) -> RedisGetResult {
        let mut store = self.data.lock().unwrap();
        if let Some(value) = store.get(key) {
            if let Some(expiry) = value.expires_at {
                if SystemTime::now() > expiry {
                    store.remove(key);
                    return RedisGetResult::Expired;
                }
            }
            RedisGetResult::Value(value.data.clone())
        } else {
            RedisGetResult::None
        }
    }
}
