use crate::search_parser::SearchParser;
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

            // Add the new value(s) to the array
            if let Value::Array(ref mut arr) = current_array {
                match new_value {
                    Value::Array(values) => arr.extend(values),
                    value => arr.push(value),
                }
                // Update the stored value
                existing.data = serde_json::to_string(&current_array)?;
            }
        } else {
            // Parse the new value as JSON
            let new_value: Value =
                serde_json::from_str(&value).map_err(|_| "New value is not valid JSON")?;

            // Create initial array - if new_value is an array, use its elements directly
            let array = match new_value {
                Value::Array(arr) => json!(arr), // This spreads the array elements
                value => json!([value]),
            };

            let value = RedisValue {
                data: serde_json::to_string(&array)?,
                expires_at: None,
            };
            store.insert(key, value);
        }
        Ok(())
    }
    pub fn get(&self, key: &str) -> RedisGetResult {
        let mut store = self.data.lock().unwrap();

        if key.contains('?') {
            let parts: Vec<&str> = key.split('?').collect();
            if parts.len() == 2 {
                if let Some(value) = store.get(parts[0]) {
                    let json_value: Value = match serde_json::from_str(&value.data) {
                        Ok(v) => v,
                        Err(_) => return RedisGetResult::None,
                    };

                    if let Value::Array(array) = json_value {
                        let conditions = SearchParser::parse_search_params(parts[1]);
                        let filtered_array: Vec<&Value> = array
                            .iter()
                            .filter(|item| SearchParser::matches_conditions(item, &conditions))
                            .collect();

                        return RedisGetResult::Value(
                            serde_json::to_string(&filtered_array).unwrap_or_default(),
                        );
                    }
                    return RedisGetResult::None;
                }
            }
        }

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
