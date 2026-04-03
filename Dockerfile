# stage 1 — use a Bookworm-based stable tag (matches runtime) and avoids missing 1.91 / bad mirror sync.
FROM rust:1.85-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# stage 2
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/rust-study .
EXPOSE 8000
CMD ["./rust-study"]
