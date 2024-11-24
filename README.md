# Rust Redis Implementation

A lightweight Redis server implementation in Rust, featuring core Redis functionality with a focus on JSON operations, query capabilities, and multi-tenant support. This project demonstrates concurrent programming, data structures, and network protocol implementation in Rust.

## Features

- 🚀 Core Redis Commands (`SET`, `GET`, `PING`, `ECHO`)
- 👥 Multi-tenant Support
  - Tenant isolation using `CLIENT SETNAME`
  - Automatic key namespacing
  - Tenant-specific data storage and queries
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
  - Tenant-aware querying

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

### Multi-tenant Operations

```bash
# Set tenant name (required before operations)
redis-cli CLIENT SETNAME tenant1

# Store data for tenant1
redis-cli SET users '[{"name":"John","age":30}]'
# Internally stored as "tenant1:users"

# Switch to different tenant
redis-cli CLIENT SETNAME tenant2

# Store data for tenant2
redis-cli SET users '[{"name":"Jane","age":25}]'
# Internally stored as "tenant2:users"

# Each tenant can only access their own data
redis-cli GET users
# Returns tenant2's data only
```

### Basic Operations

```bash
# Using redis-cli (after setting tenant name)
redis-cli PING
# Response: PONG

redis-cli SET mykey "Hello, Redis!"
# Response: OK

redis-cli GET mykey
# Response: "Hello, Redis!"
```

### JSON Operations with Tenant Isolation

```bash
# Set tenant name
redis-cli CLIENT SETNAME tenant1

# Store JSON array
redis-cli SET users '[{"name":"John","age":30},{"name":"Jane","age":25}]'

# Query with parameters (tenant-specific)
redis-cli GET "users?name=John"
# Response: [{"name":"John","age":30}]

# Append to JSON array
redis-cli APPEND users '{"name":"Bob","age":35}'
```

### Expiration

```bash
# Set key with 1000ms expiration (tenant-specific)
redis-cli SET tempkey "temporary" PX 1000

# Key will return null after expiration
redis-cli GET tempkey
```

## Implementation Details

### Architecture

- **Store Module**: Thread-safe key-value store using `Arc<Mutex<HashMap>>`
- **Parser Module**: RESP protocol parser for Redis command parsing
- **Handler Module**: Async connection handler with tenant management using Tokio
- **Types Module**: Core data structures and enums

### Key Components

1. **RedisStore**
   - Handles data storage and retrieval
   - Implements JSON operations and filtering
   - Manages key expiration
   - Enforces tenant isolation

2. **Connection Handler**
   - Manages tenant context
   - Handles tenant name setting
   - Enforces tenant-based access control
   - Implements key namespacing

3. **Command Parser**
   - Implements RESP (Redis Serialization Protocol)
   - Handles various command formats
   - Robust error handling

## Testing

The project includes comprehensive test coverage:

```bash
cargo test
```

Test suites include:
- Multi-tenant operations
- Tenant isolation
- Tenant-specific queries
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
- Minimal overhead for tenant isolation

## Security Features

- Strict tenant isolation
- Required tenant identification
- Automatic key namespacing
- Prevention of cross-tenant access

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## Credits

- Inspired by Redis
- Built with Rust and Tokio
- Uses serde_json for JSON handling
