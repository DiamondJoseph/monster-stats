FROM rust:slim AS build

RUN rustup target add x86_64-unknown-linux-musl && \
    apt-get update && \
    apt-get install -y musl-tools musl-dev && \
    update-ca-certificates

WORKDIR /build

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./main.rs ./main.rs

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

LABEL org.opencontainers.image.source=https://github.com/DiamondJoseph/monster-stats.git
LABEL org.opencontainers.image.licenses=MIT

COPY --from=build /build/target/x86_64-unknown-linux-musl/release/monster-stats /app/monster-stats

CMD ["serve"]
ENTRYPOINT ["/app/monster-stats"]
