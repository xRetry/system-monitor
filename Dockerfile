FROM rust:1.67.0 as builder
WORKDIR /usr/src/system-monitor
COPY . .
RUN cargo install --path .
 
FROM debian:buster-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/system-monitor /usr/local/bin/system-monitor
CMD ["system-monitor"]
