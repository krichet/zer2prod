# FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
# WORKDIR /app
# FROM chef AS planner
# COPY . .
# RUN cargo chef prepare --recipe-path recipe.json
# FROM chef AS builder
# COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
FROM --platform=$TARGETPLATFORM rust:latest as builder
WORKDIR /app
# Build applicationn
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release
# Runtime stage

FROM --platform=$TARGETPLATFORM debian:bookworm-slim AS runtime
RUN uname -a
RUN apt-get update -y \
    && apt-get install curl -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
# Copy the compiled binary from the builder environment
# to our runtime environment
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration.yaml .
COPY configuration.production.yaml .
ENV APP_ENVIRONMENT production
ENV POSTGRES_USERNAME=$Postgres.username \
    POSTGRES_PASSWORD=$Postgres.password \
    POSTGRES_HOST=$Postgres.host \
    POSTGRES_PORT=$Postgres.port \
    POSTGRES_DBNAME=$Postgres.dbname
ENTRYPOINT ["./zero2prod"]
