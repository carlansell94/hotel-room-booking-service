# Building Stage
FROM rust:latest AS building
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates
WORKDIR /room_booking
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release

# Deployment Stage
FROM alpine:latest
ENV ROCKET_ADDRESS=0.0.0.0
WORKDIR /
COPY --from=building /room_booking/target/x86_64-unknown-linux-musl/release/room_booking_service ./
CMD ./room_booking_service

EXPOSE 8000
LABEL org.opencontainers.image.description="Hotel room booking service container"
LABEL org.opencontainers.image.licenses=GPL-3.0-or-later
