# build stage
FROM rust:1.91-alpine AS builder
WORKDIR /app

RUN apk add --no-cache musl-dev

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# runtime stage
FROM gcr.io/distroless/cc
WORKDIR /app

COPY --from=builder /app/target/release/jwks-server /jwks-server

EXPOSE 80
ENTRYPOINT ["/jwks-server"]
VOLUME ["/jwks"]
