# Rust Hotel Room Booking Service
A hotel room booking management service written in rust.

## Features:
* Add new bookings
* List all bookings
* Get bookings by room type/customer

This service does not currently feature long-term storage, with data stored in a ```HashMap```. This means any data held by the service will be lost once the service stops.

This service is intended to be used as part of a larger microservice-based hotel booking management application.

## Dependencies:
The main dependencies of this service are:
* Rocket (web server)
* Okapi (OpenAPI/SwaggerUI)

A full list of dependencies can be found in ```Cargo.toml```.

## How To Use:
Clone the repository and build the service using cargo. In the root project directory, run

```
cargo build --release
```

The built binary file can then be found at ```./target/release/room_booking```.

## Interface
The service exposes an OpenAPI schema using SwaggerUI. To view this, navigate to ```{ip:port}/swagger-ui``` in your browser.

For example, if the service is running on localhost on the default port, the correct path will be ```http://127.0.0.1:8000/swagger-ui```