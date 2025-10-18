# syntax=docker/dockerfile:1.7
FROM rust:1.80-alpine AS build
RUN apk add --no-cache musl-dev pkgconfig openssl-dev
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl || \
    (echo "Falling back to glibc build" && cargo build --release)

FROM scratch
COPY --from=build /src/target/*/release/hello-ai-cli /hello-ai-cli
ENTRYPOINT ["/hello-ai-cli"]