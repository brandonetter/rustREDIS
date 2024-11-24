use redis_test_simple::store::RedisStore;
use redis_test_simple::types::RedisGetResult;
use serde_json::json;

#[test]
fn test_tenant_isolation() {
    let store = RedisStore::new();

    // Two tenants setting same key name
    store
        .set(
            "tenant1:users".to_string(),
            "[{\"name\":\"John\"}]".to_string(),
            None,
        )
        .unwrap();
    store
        .set(
            "tenant2:users".to_string(),
            "[{\"name\":\"Jane\"}]".to_string(),
            None,
        )
        .unwrap();

    // Verify each tenant gets their own data
    match store.get("tenant1:users") {
        RedisGetResult::Value(val) => assert_eq!(val, "[{\"name\":\"John\"}]"),
        _ => panic!("Expected tenant1 data"),
    }

    match store.get("tenant2:users") {
        RedisGetResult::Value(val) => assert_eq!(val, "[{\"name\":\"Jane\"}]"),
        _ => panic!("Expected tenant2 data"),
    }

    // Verify tenants can't access each other's data
    match store.get("tenant1:other_users") {
        RedisGetResult::None => (), // Expected
        _ => panic!("Should not find non-existent tenant key"),
    }
}

#[test]
fn test_tenant_search() {
    let store = RedisStore::new();

    // Set up test data for two tenants
    let tenant1_data = json!([
        {"name": "John", "age": 30},
        {"name": "Alice", "age": 25}
    ])
    .to_string();

    let tenant2_data = json!([
        {"name": "John", "age": 35},
        {"name": "Bob", "age": 28}
    ])
    .to_string();

    store
        .set("tenant1:users".to_string(), tenant1_data, None)
        .unwrap();
    store
        .set("tenant2:users".to_string(), tenant2_data, None)
        .unwrap();

    // Search within tenant1's data
    match store.get("tenant1:users?age_gt=28") {
        RedisGetResult::Value(val) => {
            let result: serde_json::Value = serde_json::from_str(&val).unwrap();
            assert_eq!(result.as_array().unwrap().len(), 1);
            assert_eq!(result[0]["name"], "John");
            assert_eq!(result[0]["age"], 30);
        }
        _ => panic!("Expected filtered tenant1 data"),
    }

    // Search within tenant2's data
    match store.get("tenant2:users?name=John") {
        RedisGetResult::Value(val) => {
            let result: serde_json::Value = serde_json::from_str(&val).unwrap();
            assert_eq!(result.as_array().unwrap().len(), 1);
            assert_eq!(result[0]["age"], 35);
        }
        _ => panic!("Expected filtered tenant2 data"),
    }
}

#[test]
fn test_tenant_append() {
    let store = RedisStore::new();

    // Initialize data for two tenants
    store
        .set(
            "tenant1:users".to_string(),
            "[{\"name\":\"John\"}]".to_string(),
            None,
        )
        .unwrap();

    store
        .set(
            "tenant2:users".to_string(),
            "[{\"name\":\"Jane\"}]".to_string(),
            None,
        )
        .unwrap();

    // Append to tenant1's data
    store
        .append(
            "tenant1:users".to_string(),
            "{\"name\":\"Alice\"}".to_string(),
        )
        .unwrap();

    // Verify append worked for tenant1 but didn't affect tenant2
    match store.get("tenant1:users") {
        RedisGetResult::Value(val) => {
            let data: serde_json::Value = serde_json::from_str(&val).unwrap();
            assert_eq!(data.as_array().unwrap().len(), 2);
            assert_eq!(data[1]["name"], "Alice");
        }
        _ => panic!("Expected tenant1 appended data"),
    }

    match store.get("tenant2:users") {
        RedisGetResult::Value(val) => {
            let data: serde_json::Value = serde_json::from_str(&val).unwrap();
            assert_eq!(data.as_array().unwrap().len(), 1);
            assert_eq!(data[0]["name"], "Jane");
        }
        _ => panic!("Expected tenant2 data unchanged"),
    }
}

#[test]
fn test_tenant_expiration() {
    let store = RedisStore::new();

    store
        .set(
            "tenant1:temp".to_string(),
            "\"tenant1 data\"".to_string(),
            Some(100), // 100ms
        )
        .unwrap();

    // verify data is present
    match store.get("tenant1:temp") {
        RedisGetResult::Value(val) => assert_eq!(val, "\"tenant1 data\""),
        _ => panic!("Expected tenant1 data"),
    }

    // Wait for expiration
    std::thread::sleep(std::time::Duration::from_millis(200));

    // Debug prints
    let result = store.get("tenant1:temp");

    assert!(matches!(result, RedisGetResult::Expired));
}
