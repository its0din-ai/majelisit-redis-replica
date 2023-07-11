FROM rust:latest as builder

WORKDIR /rediz
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release


FROM debian:latest

WORKDIR /rediz
COPY --from=builder /rediz/target/release/rediz .


EXPOSE 6379
CMD ["./rediz"]