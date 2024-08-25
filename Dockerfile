FROM rust:1.79.0-alpine AS builder

WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

RUN apk add --no-cache musl-dev libressl-dev

COPY . .

RUN RUSTFLAGS='-C link-arg=-s' cargo build --release --target x86_64-unknown-linux-musl

FROM gcr.io/distroless/cc-debian11

COPY --from=builder /usr/src/target/x86_64-unknown-linux-musl/release/connection-holder /usr/local/bin/connection-holder

EXPOSE 8080

CMD ["/usr/local/bin/connection-holder"]