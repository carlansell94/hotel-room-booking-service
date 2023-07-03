FROM rust:latest
WORKDIR /room_booking
COPY . .
RUN cargo install --path .
ENV ROCKET_ADDRESS=0.0.0.0
CMD room_booking_service
