FROM rust as builder
WORKDIR /usr/src/miradors
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y pkg-config libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/miradors /usr/local/bin/miradors
CMD ["miradors"]
