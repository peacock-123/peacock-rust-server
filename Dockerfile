FROM rust:latest AS builder

RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/peacock

COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/peacock/target/release/peacock-server .

EXPOSE 10000

CMD ["./peacock-server"]
