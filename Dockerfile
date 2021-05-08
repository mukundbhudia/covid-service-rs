FROM rust:1.52 as builder
WORKDIR /usr/src/covid-service-rs
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/covid-service-rs /usr/local/bin/covid-service-rs
CMD ["covid-service-rs"]