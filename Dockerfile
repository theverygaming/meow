FROM rust:1-slim-bookworm AS builder-rs
WORKDIR /app
COPY ./api .
RUN apt-get update && apt-get install -y libpq-dev && apt-get clean
RUN cargo install --path .

FROM node:22-alpine AS builder-js
WORKDIR /usr/src/app
# install deps first, then copy source for better layers
COPY ./frontend/package.json ./
COPY ./frontend/package-lock.json ./
RUN npm install
# now copy source
COPY ./frontend ./
# now build
RUN npm run build

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq5 && apt-get clean
COPY --from=builder-rs /usr/local/cargo/bin/api /usr/local/bin/api
COPY --from=builder-js /usr/src/app/dist /static
ENV ROCKET_ADDRESS=0.0.0.0
ENV STATIC_DIR=/static
EXPOSE 8000
CMD ["api"]
