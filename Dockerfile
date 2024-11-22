FROM rust:1.75-slim as builder

WORKDIR /usr/src/app
COPY . .

# Build with optimization
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin
COPY --from=builder /usr/src/app/target/release/redis_test_simple .

ENV PORT=6379
ENV HOST=0.0.0.0

EXPOSE 6379

CMD ["redis_test_simple"]
