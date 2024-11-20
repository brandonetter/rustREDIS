use std::time::SystemTime;

#[derive(Clone)]
pub struct RedisValue {
    pub data: String,
    pub expires_at: Option<SystemTime>,
}

#[derive(Debug)]
pub enum RedisGetResult {
    Value(String),
    None,
    Expired,
}

#[derive(Debug)]
pub enum ParserState {
    Start,
    ReadingArrayLength(String),
    ExpectingBulkString,
    ReadingBulkLength(String),
    ReadingBulkContent {
        expected_length: i32,
        current_content: String,
    },
}
