FROM rust:latest AS builder

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

WORKDIR /usr/src/keymaster
COPY Cargo.lock Cargo.toml ./
COPY crates ./crates

RUN cd crates/frontend && trunk build --release
RUN SQLX_OFFLINE=true cargo build --bin backend --release

FROM gcr.io/distroless/cc-debian10

COPY --from=builder /usr/src/keymaster/target/release/backend /opt/keymaster/backend
COPY --from=builder /usr/src/keymaster/crates/frontend/dist /opt/keymaster/dist

WORKDIR /opt/keymaster
CMD ["./backend"]
