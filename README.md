# Rust Redis Implementation

A lightweight Redis server implementation in Rust, featuring core Redis functionality with a focus on JSON operations, query capabilities, and multi-tenant support. This project demonstrates concurrent programming, data structures, and network protocol implementation in Rust.

## Features

- 🚀 Core Redis Commands (`SET`, `GET`, `PING`, `ECHO`)
- 👥 Multi-tenant Support
  - Tenant isolation using `CLIENT SETNAME`
  - Automatic key namespacing
  - Tenant-specific data storage
- 📊 JSON Data Support
  - Automatic JSON parsing and validation
  - Array operations with `APPEND`
  - Query filtering with URL-style parameters
- 📈 Automatic Performance Metrics
  - Operation timing and sizing
  - Query pattern analysis
  - Per-tenant statistics

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

## Basic Usage

### Multi-tenant Operations

```bash
# Set tenant name (required before operations)
redis-cli CLIENT SETNAME tenant1

# Store JSON data
redis-cli SET users '[{"name":"John","age":30},{"name":"Jane","age":25}]'

# Each tenant's data is isolated
redis-cli CLIENT SETNAME tenant2
redis-cli SET users '[{"name":"Alice","age":28}]'
```

### JSON Operations & Querying

```bash
# Store JSON array
redis-cli SET users '[{"name":"John","age":30}]'

# Append to array
redis-cli APPEND users '{"name":"Jane","age":25}'

# Query with filters
redis-cli GET "users?name=John"
redis-cli GET "users?age_gt=25"
redis-cli GET "users?name_like=ja"
```

## Performance Analysis

Every operation is automatically tracked in a tenant-specific `_metrics` store, providing real-time insight into performance and usage patterns.

### Analyzing Metrics

```bash
# View all metrics
redis-cli GET _metrics

# Find slow operations (>0.2ms)
redis-cli GET "_metrics?ms_gt=0.2"

# Analyze query patterns
redis-cli GET "_metrics?endpoint_like=test?"     # All filtered queries
redis-cli GET "_metrics?endpoint=test"           # Direct gets
redis-cli GET "_metrics?bytes_gt=1000"          # Large responses
redis-cli GET "_metrics?method=APPEND"          # APPEND operations

# Time window analysis
redis-cli GET "_metrics?unix_gt=1732439700&unix_lt=1732439800"

# Complex analysis
redis-cli GET "_metrics?endpoint_like=test?&ms_gt=0.2"  # Slow filtered queries
```

Each metrics entry provides detailed operation information:
```json
{
  "unix": 1732439700,
  "endpoint": "users?age_gt=30",
  "method": "GET",
  "bytes": 1351,
  "ms": 0.213
}
```

### Performance Characteristics

Based on metrics analysis, typical operation times:
- Direct GET: ~0.05ms
- JSON filtered queries: ~0.2ms
- APPEND operations: ~0.3ms
- Metrics queries: ~0.04ms

The metrics system reveals:
- Impact of JSON filtering (3-4x overhead)
- Sub-millisecond operation timing
- Query pattern performance
- Response size implications
- Command parsing overhead

## Implementation Details

### Architecture

- **Store Module**: Thread-safe key-value store using `Arc<Mutex<HashMap>>`
- **Parser Module**: RESP protocol parser
- **Handler Module**: Async connection handler with metrics collection
- **Metrics Module**: Automatic performance tracking
- **Types Module**: Core data structures and enums

### Key Components

1. **Command Handler**
   - Async operation handling
   - Tenant context management
   - Response formatting

2. **Query Engine**
   - Support for exact match and 'like' queries
   - Range queries for numeric values
   - Used for both data and metrics analysis

3. **Metrics Collection**
   - Zero-overhead tenant isolation
   - Microsecond precision timing
   - Automatic query pattern analysis

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

