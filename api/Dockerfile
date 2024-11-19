FROM rust:1-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y libpq-dev && apt-get clean
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq5 && apt-get clean
COPY --from=builder /usr/local/cargo/bin/api /usr/local/bin/api
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["api"]
