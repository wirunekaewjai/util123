# Build stage
FROM rust:bookworm AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

# Final run stage
FROM gcr.io/distroless/cc-debian12 AS runner

WORKDIR /app

COPY --from=builder /app/target/release/util123 /app/util123
COPY --from=builder /app/assets /app/assets
COPY --from=builder /app/public /app/public

CMD ["/app/util123"]