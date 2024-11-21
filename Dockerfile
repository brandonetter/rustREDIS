FROM rust:1.75-slim as builder

WORKDIR /usr/src/app
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install necessary runtime libraries
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -U -s /bin/bash redisuser

WORKDIR /usr/local/bin
COPY --from=builder /usr/src/app/target/release/redis_test_simple .

# Set ownership
RUN chown redisuser:redisuser /usr/local/bin/redis_test_simple

# Switch to non-root user
USER redisuser

# Explicitly set environment variables
ENV PORT=10000
ENV HOST=0.0.0.0

# Expose the port
EXPOSE 10000

CMD ["redis_test_simple"]
