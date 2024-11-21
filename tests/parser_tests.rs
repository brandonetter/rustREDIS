use redis_test_simple::parser::parse_command;

#[test]
fn test_simple_command() {
    // Tests simple SET command: SET key value
    let input = "*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n";
    let result = parse_command(input);
    assert_eq!(result, vec!["SET", "key", "value"]);
}

#[test]
fn test_empty_array() {
    // Tests empty command array
    let input = "*0\r\n";
    let result = parse_command(input);
    assert_eq!(result.len(), 0);
}

#[test]
fn test_ping_command() {
    // Tests simple PING command
    let input = "*1\r\n$4\r\nPING\r\n";
    let result = parse_command(input);
    assert_eq!(result, vec!["PING"]);
}

#[test]
fn test_special_characters() {
    // Tests handling of special characters in strings
    let input = "*3\r\n$3\r\nSET\r\n$4\r\nkey:\r\n$6\r\nval@ue\r\n";
    let result = parse_command(input);
    assert_eq!(result, vec!["SET", "key:", "val@ue"]);
}

#[test]
fn test_malformed_input() {
    // Tests various malformed inputs
    let test_cases = vec![
        // Missing array length
        "*\r\n",
        // Invalid array length
        "*-1\r\n",
        // Missing bulk string length
        "*1\r\n$\r\n",
        // Content length mismatch
        "*1\r\n$5\r\ntest\r\n",
    ];

    for case in test_cases {
        let result = parse_command(case);
        assert_eq!(result.len(), 0, "Malformed input should return empty vec");
    }
}

#[test]
fn test_complex_command() {
    // Tests more complex command with longer strings
    let input = "*5\r\n$4\r\nMGET\r\n$3\r\nkey\r\n$4\r\nkey2\r\n$4\r\nkey3\r\n$4\r\nkey4\r\n";
    let result = parse_command(input);
    assert_eq!(result, vec!["MGET", "key", "key2", "key3", "key4"]);
}

#[test]
fn test_empty_strings() {
    // Tests handling of empty strings
    let input = "*3\r\n$3\r\nSET\r\n$0\r\n\r\n$0\r\n\r\n";
    let result = parse_command(input);
    assert_eq!(result, vec!["SET", "", ""]);
}

#[test]
fn test_state_transitions() {
    // This test verifies the parser moves through states correctly
    let input = "*2\r\n$4\r\nECHO\r\n$5\r\nhello\r\n";
    let result = parse_command(input);
    assert_eq!(result, vec!["ECHO", "hello"]);

    // We could also test intermediate states if we modify the parser
    // to expose state information for testing
}

#[test]
fn test_consecutive_commands() {
    // Tests parsing multiple commands in sequence
    let input1 = "*1\r\n$4\r\nPING\r\n";
    let input2 = "*2\r\n$4\r\nECHO\r\n$5\r\nhello\r\n";

    let result1 = parse_command(input1);
    let result2 = parse_command(input2);

    assert_eq!(result1, vec!["PING"]);
    assert_eq!(result2, vec!["ECHO", "hello"]);
}

#[test]
fn test_whitespace_handling() {
    // Tests handling of whitespace in values
    let input = "*3\r\n$3\r\nSET\r\n$5\r\nkey 1\r\n$7\r\nvalue 1\r\n";
    let result = parse_command(input);
    assert_eq!(result, vec!["SET", "key 1", "value 1"]);
}
