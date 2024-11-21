# Rust Redis Implementation

A lightweight Redis server implementation in Rust, featuring core Redis functionality with a focus on JSON operations and query capabilities. This project demonstrates concurrent programming, data structures, and network protocol implementation in Rust.

## Features

- 🚀 Core Redis Commands (`SET`, `GET`, `PING`, `ECHO`)
- 📊 JSON Data Support
  - Automatic JSON parsing and validation
  - Array operations with `APPEND`
  - Query filtering with URL-style parameters
- ⏰ Time-Based Operations
  - Key expiration (PX option)
  - Automatic cleanup of expired keys
- 🔄 Concurrent Operations
  - Thread-safe data store using `Arc` and `Mutex`
  - Async I/O with Tokio
- 🔍 Query Parameters
  - Filter JSON arrays using key-value pairs
  - Support for multiple search conditions

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/rust-redis-implementation
cd rust-redis-implementation

# Build the project
cargo build --release

# Run the tests
cargo test

# Start the server
cargo run
```

The server will start listening on `127.0.0.1:6379`.

## Usage Examples

### Basic Operations

```bash
# Using redis-cli
redis-cli PING
# Response: PONG

redis-cli SET mykey "Hello, Redis!"
# Response: OK

redis-cli GET mykey
# Response: "Hello, Redis!"
```

### JSON Operations

```bash
# Store JSON array
redis-cli SET users '[{"name":"John","age":30},{"name":"Jane","age":25}]'

# Query with parameters
redis-cli GET "users?name=John"
# Response: [{"name":"John","age":30}]

# Append to JSON array
redis-cli APPEND users '{"name":"Bob","age":35}'
```

### Expiration

```bash
# Set key with 1000ms expiration
redis-cli SET tempkey "temporary" PX 1000

# Key will return null after expiration
redis-cli GET tempkey
```

## Implementation Details

### Architecture

- **Store Module**: Thread-safe key-value store using `Arc<Mutex<HashMap>>`
- **Parser Module**: RESP protocol parser for Redis command parsing
- **Handler Module**: Async connection handler using Tokio
- **Types Module**: Core data structures and enums

### Key Components

1. **RedisStore**
   - Handles data storage and retrieval
   - Implements JSON operations and filtering
   - Manages key expiration

2. **Command Parser**
   - Implements RESP (Redis Serialization Protocol)
   - Handles various command formats
   - Robust error handling

3. **Connection Handler**
   - Async TCP connection management
   - Command routing and execution
   - Response formatting

## Testing

The project includes comprehensive test coverage:

```bash
cargo test
```

Test suites include:
- Basic operations (SET/GET)
- JSON operations
- Expiration handling
- Parser edge cases
- Connection handling
- Concurrent operations

## Performance Considerations

- Uses Tokio for async I/O operations
- Thread-safe concurrent access to the store
- Efficient JSON parsing and filtering
- Automatic cleanup of expired keys

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.


- Inspired by Redis
- Built with Rust and Tokio
- Uses serde_json for JSON handling
