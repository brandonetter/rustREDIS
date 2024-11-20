use crate::types::{RedisGetResult, RedisValue};
use serde_json::{json, Value};
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

    pub fn append(&self, key: String, value: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut store = self.data.lock().unwrap();

        if let Some(existing) = store.get_mut(&key) {
            // Parse the existing data as JSON array
            let mut current_array: Value = serde_json::from_str(&existing.data)
                .map_err(|_| "Existing data is not a valid JSON array")?;

            // Parse the new value as JSON
            let new_value: Value =
                serde_json::from_str(&value).map_err(|_| "New value is not valid JSON")?;

            // Ensure we have an array
            if !current_array.is_array() {
                return Err("Existing data is not a JSON array".into());
            }

            // Add the new value to the array
            if let Value::Array(ref mut arr) = current_array {
                arr.push(new_value);
                // Update the stored value
                existing.data = serde_json::to_string(&current_array)?;
            }
        } else {
            // If key doesn't exist, create new array with single element
            let new_value: Value =
                serde_json::from_str(&value).map_err(|_| "New value is not valid JSON")?;

            let array = json!([new_value]);
            let value = RedisValue {
                data: array.to_string(),
                expires_at: None,
            };
            store.insert(key, value);
        }
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
