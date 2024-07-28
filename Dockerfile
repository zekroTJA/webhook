FROM --platform=$BUILDPLATFORM docker.io/library/rust:slim AS build
WORKDIR /build
COPY src/ src/
COPY Cargo.toml .
COPY Cargo.lock .
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo build --release

FROM debian:12-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libssl-dev ca-certificates
COPY --from=build /build/target/release/webhook /app/webhook
ENTRYPOINT [ "/app/webhook" ]