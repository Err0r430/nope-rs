FROM rust:1.86-slim AS builder


RUN apt-get update \
  && apt-get install -y curl ca-certificates pkg-config libssl-dev \
 && curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
 && apt-get install -y nodejs

WORKDIR /app

COPY . .

RUN cargo build --release

# Use smaller base image for runtime
FROM debian:bookworm-slim


RUN apt-get update \
  && apt-get install -y libssl3 ca-certificates \
  && rm -rf /var/lib/apt/lists/*


WORKDIR /app
COPY --from=builder /app/target/release/nope-rs .

CMD ["./nope-rs"]