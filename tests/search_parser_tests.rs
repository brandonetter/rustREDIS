use redis_test_simple::search_parser::{FilterCondition, FilterOperator, SearchParser};
use serde_json::json;

#[test]
fn test_parse_simple_equals() {
    let params = SearchParser::parse_search_params("name=John");
    assert_eq!(params.len(), 1);
    assert_eq!(params[0].field, "name");
    assert_eq!(params[0].operator, FilterOperator::Equals);
    assert_eq!(params[0].value, "John");
}

#[test]
fn test_parse_url_encoded_values() {
    let params = SearchParser::parse_search_params(
        "name=John%20Smith&description=Some%20text%20with%20spaces",
    );
    assert_eq!(params.len(), 2);
    assert_eq!(params[0].value, "John Smith");
    assert_eq!(params[1].value, "Some text with spaces");
}

#[test]
fn test_parse_special_characters() {
    let params = SearchParser::parse_search_params("query=hello%26world&tag=%23rust");
    assert_eq!(params.len(), 2);
    assert_eq!(params[0].value, "hello&world");
    assert_eq!(params[1].value, "#rust");
}

#[test]
fn test_parse_numeric_comparisons() {
    let params = SearchParser::parse_search_params("age_gt=20&price_lt=100");
    assert_eq!(params.len(), 2);
    assert_eq!(params[0].operator, FilterOperator::GreaterThan);
    assert_eq!(params[1].operator, FilterOperator::LessThan);
}

#[test]
fn test_matches_conditions() {
    let item = json!({
        "name": "John Smith",
        "age": 30,
        "city": "New York"
    });

    let conditions = vec![
        FilterCondition {
            field: "age".to_string(),
            operator: FilterOperator::GreaterThan,
            value: "25".to_string(),
        },
        FilterCondition {
            field: "name".to_string(),
            operator: FilterOperator::Like,
            value: "john".to_string(),
        },
    ];

    assert!(SearchParser::matches_conditions(&item, &conditions));
}

#[test]
fn test_malformed_url_encoding() {
    let params = SearchParser::parse_search_params("name=%XX&valid=test");
    assert_eq!(params.len(), 1, "Should skip malformed URL encoded value");
    assert_eq!(params[0].field, "valid");
    assert_eq!(params[0].value, "test");
}

#[test]
fn test_complex_url_encoding() {
    let params = SearchParser::parse_search_params(
        "invalid=%XX&name=John%20Smith%20%26%20Co&bad=%Y&valid=test%20case",
    );
    assert_eq!(params.len(), 2, "Should only include valid encoded values");
    assert!(params
        .iter()
        .any(|p| p.field == "name" && p.value == "John Smith & Co"));
    assert!(params
        .iter()
        .any(|p| p.field == "valid" && p.value == "test case"));
}

#[test]
fn test_numeric_comparisons() {
    let item = json!({
        "age": 30,
        "price": 99.99
    });

    let test_cases = vec![
        ("age_gt=25", true),
        ("age_lt=35", true),
        ("age_gte=30", true),
        ("age_lte=30", true),
        ("price_lt=100", true),
        ("price_gt=100", false),
    ];

    for (query, expected) in test_cases {
        let conditions = SearchParser::parse_search_params(query);
        assert_eq!(
            SearchParser::matches_conditions(&item, &conditions),
            expected,
            "Failed for query: {}",
            query
        );
    }
}

#[test]
fn test_like_operator() {
    let item = json!({
        "name": "John Smith",
        "description": "Some long text here"
    });

    let test_cases = vec![
        ("name_like=john", true),
        ("name_like=smith", true),
        ("name_like=jane", false),
        ("description_like=long%20text", true),
        ("description_like=missing", false),
    ];

    for (query, expected) in test_cases {
        let conditions = SearchParser::parse_search_params(query);
        assert_eq!(
            SearchParser::matches_conditions(&item, &conditions),
            expected,
            "Failed for query: {}",
            query
        );
    }
}
