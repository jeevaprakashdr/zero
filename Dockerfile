FROM rustlang/rust:nightly-bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    && rm -rf /var/lib/apt/lists/*
    
RUN rustup target add x86_64-unknown-linux-gnu

WORKDIR /app

COPY . .
