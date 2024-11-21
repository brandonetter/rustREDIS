use redis_test_simple::store::RedisStore;
use redis_test_simple::types::RedisGetResult;

use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_basic_set_get() {
    let store = RedisStore::new();

    // Test setting and getting a simple value
    store
        .set("test_key".to_string(), "test_value".to_string(), None)
        .unwrap();

    match store.get("test_key") {
        RedisGetResult::Value(val) => assert_eq!(val, "test_value"),
        _ => panic!("Expected Value variant"),
    }
}

#[test]
fn test_get_nonexistent_key() {
    let store = RedisStore::new();
    match store.get("nonexistent") {
        RedisGetResult::None => (), // Expected behavior
        _ => panic!("Expected None variant for nonexistent key"),
    }
}

#[test]
fn test_expiration() {
    let store = RedisStore::new();

    // Set with 100ms expiry
    store
        .set(
            "expire_key".to_string(),
            "expire_value".to_string(),
            Some(100), // 100ms
        )
        .unwrap();

    // Should exist immediately
    match store.get("expire_key") {
        RedisGetResult::Value(val) => assert_eq!(val, "expire_value"),
        _ => panic!("Expected Value variant immediately after setting"),
    }

    // Wait for expiration
    sleep(Duration::from_millis(150));

    // Should be expired now
    match store.get("expire_key") {
        RedisGetResult::Expired => (), // Expected behavior
        _ => panic!("Expected Expired variant after ttl"),
    }
}

#[test]
fn test_json_append() {
    let store = RedisStore::new();

    // Initial JSON array
    store
        .set("json_key".to_string(), "[1,2,3]".to_string(), None)
        .unwrap();

    // Append new value
    store
        .append("json_key".to_string(), "4".to_string())
        .unwrap();

    // Check final array
    match store.get("json_key") {
        RedisGetResult::Value(val) => assert_eq!(val, "[1,2,3,4]"),
        _ => panic!("Expected Value variant with appended array"),
    }
}

#[test]
fn test_json_append_to_nonexistent_key() {
    let store = RedisStore::new();

    // Append to non-existent key should create new array
    store
        .append("new_json_key".to_string(), "1".to_string())
        .unwrap();

    match store.get("new_json_key") {
        RedisGetResult::Value(val) => assert_eq!(val, "[1]"),
        _ => panic!("Expected Value variant with new array"),
    }
}

#[test]
fn test_invalid_json_append() {
    let store = RedisStore::new();

    // Set initial valid JSON
    store
        .set("json_key".to_string(), "[1,2,3]".to_string(), None)
        .unwrap();

    // Try to append invalid JSON
    let result = store.append("json_key".to_string(), "not valid json".to_string());

    assert!(result.is_err());
}
