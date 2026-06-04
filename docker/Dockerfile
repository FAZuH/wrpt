FROM rust:1.85-slim AS build
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock /app/
COPY src/ /app/src/
RUN cargo build --release


FROM debian:12-slim AS final

RUN apt-get update && apt-get install -y --no-install-recommends openssl ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 autoport

COPY --from=build /app/target/release/autoport /usr/bin/
COPY --from=docker/compose-bin:v2.32.4 /docker-compose /usr/bin/docker-compose

USER autoport

ENTRYPOINT ["/usr/bin/autoport"]
