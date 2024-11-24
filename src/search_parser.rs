use percent_encoding::percent_decode_str;
use serde_json::Value;

#[derive(Debug, PartialEq)]
pub enum FilterOperator {
    Equals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Like,
    NotEquals,
}

#[derive(Debug)]
pub struct FilterCondition {
    pub field: String,
    pub operator: FilterOperator,
    pub value: String,
}

pub struct SearchParser;

impl SearchParser {
    fn is_valid_percent_encoding(s: &str) -> bool {
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '%' {
                // Need two more valid hex digits after %
                match (chars.next(), chars.next()) {
                    (Some(d1), Some(d2)) => {
                        if !d1.is_ascii_hexdigit() || !d2.is_ascii_hexdigit() {
                            return false;
                        }
                    }
                    _ => return false,
                }
            }
        }
        true
    }

    pub fn parse_search_params(query_string: &str) -> Vec<FilterCondition> {
        query_string
            .split('&')
            .filter_map(|param| {
                let parts: Vec<&str> = param.split('=').collect();
                if parts.len() != 2 {
                    return None;
                }

                if !Self::is_valid_percent_encoding(parts[1]) {
                    return None;
                }

                let decoded_value = match percent_decode_str(parts[1]).decode_utf8() {
                    Ok(decoded) => decoded.replace("+", " "),
                    Err(_) => return None,
                };

                println!("Decoded value: {}", decoded_value);

                Self::parse_field_and_operator(parts[0], &decoded_value)
            })
            .collect()
    }

    fn parse_field_and_operator(field_with_op: &str, value: &str) -> Option<FilterCondition> {
        let parts: Vec<&str> = field_with_op.split('_').collect();
        match parts.len() {
            1 => Some(FilterCondition {
                field: parts[0].to_string(),
                operator: FilterOperator::Equals,
                value: value.to_string(),
            }),
            2 => {
                let operator = match parts[1] {
                    "gt" => Some(FilterOperator::GreaterThan),
                    "lt" => Some(FilterOperator::LessThan),
                    "gte" => Some(FilterOperator::GreaterThanOrEqual),
                    "lte" => Some(FilterOperator::LessThanOrEqual),
                    "like" => Some(FilterOperator::Like),
                    "ne" => Some(FilterOperator::NotEquals),
                    _ => None,
                };

                operator.map(|op| FilterCondition {
                    field: parts[0].to_string(),
                    operator: op,
                    value: value.to_string(),
                })
            }
            _ => None,
        }
    }

    pub fn matches_conditions(item: &Value, conditions: &[FilterCondition]) -> bool {
        conditions.iter().all(|condition| {
            if let Some(field_value) = item.get(&condition.field) {
                Self::compare_values(field_value, &condition.value, &condition.operator)
            } else {
                false
            }
        })
    }

    fn compare_values(field: &Value, search_value: &str, operator: &FilterOperator) -> bool {
        match operator {
            FilterOperator::Equals => Self::values_equal(field, search_value),
            FilterOperator::NotEquals => !Self::values_equal(field, search_value),
            FilterOperator::GreaterThan => Self::compare_numeric(field, search_value, |a, b| a > b),
            FilterOperator::LessThan => Self::compare_numeric(field, search_value, |a, b| a < b),
            FilterOperator::GreaterThanOrEqual => {
                Self::compare_numeric(field, search_value, |a, b| a >= b)
            }
            FilterOperator::LessThanOrEqual => {
                Self::compare_numeric(field, search_value, |a, b| a <= b)
            }
            FilterOperator::Like => Self::values_like(field, search_value),
        }
    }

    fn values_equal(field: &Value, search_value: &str) -> bool {
        match serde_json::from_str::<Value>(search_value) {
            Ok(search_value) => field == &search_value,
            Err(_) => matches!(field, Value::String(s) if s == search_value),
        }
    }

    fn values_like(field: &Value, search_value: &str) -> bool {
        match field {
            Value::String(s) => s.to_lowercase().contains(&search_value.to_lowercase()),
            _ => false,
        }
    }

    fn compare_numeric<F>(field: &Value, search_value: &str, comparator: F) -> bool
    where
        F: Fn(f64, f64) -> bool,
    {
        if let Ok(search_num) = search_value.parse::<f64>() {
            match field {
                Value::Number(n) => {
                    if let Some(n) = n.as_f64() {
                        return comparator(n, search_num);
                    }
                }
                _ => (),
            }
        }
        false
    }
}
