FROM rust:alpine AS build
WORKDIR /build
COPY src/ src/
COPY Cargo.toml .
COPY Cargo.lock .
RUN apk add -q --no-cache build-base openssl-dev
RUN cargo build --release

FROM alpine
WORKDIR /app
COPY --from=build /build/target/release/webhook /app/webhook
ENTRYPOINT [ "/app/webhook" ]
