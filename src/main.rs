/*
    SPDX-License-Identifier: GPL-3.0-or-later
    SPDX-FileCopyrightText: Copyright © 2023 Carl Ansell <@carlansell94>
*/

use rocket::http::Status;
use rocket::{delete, get, post, put, serde::json::Json};
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};

mod storage;
use storage::room_booking::*;

#[doc(hidden)]
/// # Create a room booking with the provided data
///
/// Creates the room booking with the provided booking data. Returns the booking.
#[openapi(tag = "Room Booking")]
#[post("/booking", format = "json", data = "<booking_details>")]
pub fn create_room_booking(
    booking_details: Json<RoomBooking>,
) -> Result<Json<RoomBooking>, Status> {
    let result: Result<RoomBooking, ()> = storage::create(booking_details.into_inner());
    match result {
        Ok(booking) => Ok(Json(booking)),
        Err(_) => Err(Status::BadRequest),
    }
}

#[doc(hidden)]
/// # Get room booking for the specified id
///
/// Returns booking details.
#[openapi(tag = "Room Booking")]
#[get("/booking/<booking_id>")]
pub fn get_room_booking(booking_id: u32) -> Result<Json<RoomBooking>, Status> {
    let result: Option<RoomBooking> = storage::fetch_by_id(booking_id);
    match result {
        Some(booking) => Ok(Json(booking)),
        None => Err(Status::NotFound),
    }
}

#[doc(hidden)]
/// # Complete the booking with the provided booking id
///
/// Sets the status of the room booking specified to 'Complete'. Returns details of the booking.
#[openapi(tag = "Room Booking")]
#[put("/booking/<booking_id>/complete")]
pub fn complete_room_booking(booking_id: u32) -> Json<bool> {
    Json(storage::status(booking_id, BookingStatus::Complete))
}

#[doc(hidden)]
/// # Cancel the booking with the provided booking id
///
/// Sets the booking status to 'Cancelled' for the booking with the provided id. Returns true on success, false on failure.
#[openapi(tag = "Room Booking")]
#[delete("/booking/<booking_id>")]
pub fn cancel_room_booking(booking_id: u32) -> Json<bool> {
    Json(storage::status(booking_id, BookingStatus::Cancelled))
}

#[doc(hidden)]
/// # Get all room bookings
///
/// Returns a list containing all room bookings in the system
#[openapi(tag = "Room Bookings")]
#[get("/bookings")]
fn get_room_bookings() -> Json<Vec<RoomBooking>> {
    return Json(storage::fetch_all());
}

#[doc(hidden)]
/// # Get room bookings for the specified customer id
///
/// Returns a list of bookings.
#[openapi(tag = "Room Bookings")]
#[get("/bookings/customer/<customer_id>")]
fn get_customer_room_bookings(customer_id: u32) -> Json<Vec<RoomBooking>> {
    return Json(storage::fetch_by_customer_id(customer_id));
}

#[doc(hidden)]
/// # Get room bookings starting on the provided date
///
/// Returns a list of bookings.
#[openapi(tag = "Room Bookings")]
#[get("/bookings/date/<date>")]
fn get_bookings_starting_on_date(date: &str) -> Json<Vec<RoomBooking>> {
    return Json(storage::fetch_by_check_in_date(date));
}

#[doc(hidden)]
/// # Get room bookings for the specified room type
///
/// Returns a list of bookings.
#[openapi(tag = "Room Bookings")]
#[get("/bookings/room-type/<room_type_id>")]
fn get_room_type_bookings(room_type_id: u8) -> Json<Vec<RoomBooking>> {
    return Json(storage::fetch_by_room_type_id(room_type_id));
}

#[doc(hidden)]
#[rocket::main]
async fn main() {
    if storage::snapshot_exists() {
        match storage::load_snapshot() {
            Ok(_) => println!("Loaded snapshot..."),
            Err(err) => println!("An error occurred loading snapshot: {}", err),
        }
    }

    let launch_result = rocket::build()
        .mount(
            "/",
            openapi_get_routes![
                get_room_booking,
                create_room_booking,
                complete_room_booking,
                cancel_room_booking,
                get_room_bookings,
                get_customer_room_bookings,
                get_bookings_starting_on_date,
                get_room_type_bookings
            ],
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .launch()
        .await;
    match launch_result {
        Ok(_) => println!("Shutdown complete."),
        Err(err) => println!("An error occurred during shutdown: {}", err),
    };
}
