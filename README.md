![Compilation](https://github.com/carlansell94/hotel-room-booking-service/actions/workflows/rust.yml/badge.svg)
![Docker Build](https://github.com/carlansell94/hotel-room-booking-service/actions/workflows/docker-publish.yml/badge.svg)

# Rust Hotel Room Booking Service

A hotel room booking management service written in rust.

## Features

* Add new bookings
* List all bookings
* Cancel bookings
* Mark bookings as completed
* Get bookings by room type/customer

Data is stored in a ```HashMap```, a copy of which is saved every time it is updated in the file ```bookings.dat```. This is automatically loaded every time the service starts.

This service is intended to be used as part of a larger microservice-based hotel booking management application.

## Dependencies

The main dependencies of this service are:

* Rocket (web server)
* Okapi (OpenAPI/SwaggerUI)

A full list of dependencies can be found in ```Cargo.toml```.

## How To Use

If you use docker, the easiest way to use the service is to use the pre-built container, which is updated on every commit.

To do this, run

```sh
docker pull ghcr.io/carlansell94/hotel-room-booking-service:master
```

It can also be used as a base image by adding

```sh
FROM ghcr.io/carlansell94/hotel-room-booking-service:master
```

to your Dockerfile.

The current Docker build status is displayed in the badge at the top of this file.

You can also clone the repository and build the service directly using cargo. In the root project directory, run

```sh
cargo build --release
```

The built binary file can then be found at ```./target/release/room_booking_service```.

## Interface

The service exposes an OpenAPI schema using SwaggerUI. To view this, navigate to ```{ip:port}/swagger-ui``` in your browser.

For example, if the service is running on localhost on the default port, the correct path will be ```http://127.0.0.1:8000/swagger-ui```

## Tests

A few tests are included, which check that bookings can be added, fetched and updated successfully. You can run these using ```cargo test```.
