FROM rust:latest as builder

WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/app usr/local/bin/app

CMD ["app"]