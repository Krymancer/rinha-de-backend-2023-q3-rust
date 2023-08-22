FROM rust:latest AS build
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY .sqlx ./.sqlx
RUN cargo build --release
RUN cargo install --path .
FROM debian:bullseye-slim
COPY --from=build /usr/local/cargo/bin/api_rust_rinha_back /usr/local/bin/api_rust_rinha_back
ENV RUST_LOG=debug
CMD ["api_rust_rinha_back"]