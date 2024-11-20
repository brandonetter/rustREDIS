use crate::types::ParserState;

pub fn parse_command(request: &str) -> Vec<String> {
    let mut state = ParserState::Start;
    let mut command_parts: Vec<String> = Vec::new();

    for ch in request.chars() {
        state = match state {
            ParserState::Start => {
                if ch == '*' {
                    ParserState::ReadingArrayLength(String::new())
                } else {
                    ParserState::Start
                }
            }
            ParserState::ReadingArrayLength(mut num) => {
                if ch == '\r' {
                    match num.parse::<i32>() {
                        Ok(_n) => ParserState::ExpectingBulkString,
                        Err(_e) => ParserState::Start,
                    }
                } else {
                    num.push(ch);
                    ParserState::ReadingArrayLength(num)
                }
            }
            ParserState::ExpectingBulkString => {
                if ch == '$' {
                    ParserState::ReadingBulkLength(String::new())
                } else if ch != '\n' {
                    ParserState::ExpectingBulkString
                } else {
                    ParserState::ExpectingBulkString
                }
            }
            ParserState::ReadingBulkLength(mut num) => {
                if ch == '\r' {
                    match num.parse::<i32>() {
                        Ok(n) => ParserState::ReadingBulkContent {
                            expected_length: n,
                            current_content: String::new(),
                        },
                        Err(_e) => ParserState::Start,
                    }
                } else {
                    num.push(ch);
                    ParserState::ReadingBulkLength(num)
                }
            }
            ParserState::ReadingBulkContent {
                expected_length,
                mut current_content,
            } => {
                if ch == '\r' {
                    if current_content.len() == expected_length as usize {
                        command_parts.push(current_content.clone());
                        ParserState::ExpectingBulkString
                    } else {
                        ParserState::ReadingBulkContent {
                            expected_length,
                            current_content,
                        }
                    }
                } else if ch != '\n' {
                    current_content.push(ch);
                    ParserState::ReadingBulkContent {
                        expected_length,
                        current_content,
                    }
                } else {
                    ParserState::ReadingBulkContent {
                        expected_length,
                        current_content,
                    }
                }
            }
        };
    }

    command_parts
}
