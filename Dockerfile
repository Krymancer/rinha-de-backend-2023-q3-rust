FROM rust:latest AS build
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release
RUN cargo install --path .
FROM debian:bullseye-slim
COPY --from=build /usr/local/cargo/bin/api_rust_rinha_back /usr/local/bin/api_rust_rinha_back
CMD ["api_rust_rinha_back"]