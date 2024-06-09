FROM rust:1-alpine3.20 as builder
RUN apk add --no-cache musl-dev protobuf-dev
WORKDIR /app
COPY ./ /app
RUN cargo build --release

FROM alpine:3.20
COPY --from=builder /app/target/release/peacock-server .
EXPOSE 50051
CMD ["./peacock-server"]
