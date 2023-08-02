# Building Stage
FROM rust:latest AS building
ENV ARCH='eval uname -m'
RUN rustup target add $($ARCH)-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates
WORKDIR /room_booking
COPY . .
RUN cargo build --target $($ARCH)-unknown-linux-musl --release
RUN mv /room_booking/target/$($ARCH)-unknown-linux-musl/release/room_booking_service /

# Deployment Stage
FROM alpine:latest
ENV ROCKET_ADDRESS=0.0.0.0
WORKDIR /
COPY --from=building /room_booking_service ./
CMD ./room_booking_service

EXPOSE 8000
LABEL org.opencontainers.image.description="Hotel room booking service container"
LABEL org.opencontainers.image.licenses=GPL-3.0-or-later
