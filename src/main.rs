/*
    SPDX-License-Identifier: GPL-3.0-or-later
    SPDX-FileCopyrightText: Copyright © 2023 Carl Ansell <@carlansell94>
*/

use rocket::http::Status;
use rocket::{get, post, serde::json::Json};
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};

mod storage;
use crate::storage::create;
use storage::room_booking::*;

/// # Create a room booking with the provided data
///
/// Creates the room booking with the provided booking data.
#[openapi(tag = "Room Booking")]
#[post("/room-booking", format = "json", data = "<booking_details>")]
pub fn create_room_booking(
    booking_details: Json<RoomBooking>,
) -> Result<Json<RoomBooking>, Status> {
    let result: Result<RoomBooking, ()> = create(booking_details.into_inner());
    match result {
        Ok(booking) => Ok(Json(booking)),
        Err(_) => Err(Status::BadRequest),
    }
}

/// # Get room booking for the specified id
///
/// Returns booking details.
#[openapi(tag = "Room Booking")]
#[get("/room-booking/<booking_id>")]
pub fn get_room_booking(booking_id: u32) -> Result<Json<RoomBooking>, Status> {
    let result: Option<RoomBooking> = storage::fetch_booking(booking_id);
    match result {
        Some(booking) => Ok(Json(booking)),
        None => Err(Status::NotFound),
    }
}

/// # Get all room bookings
///
/// Returns a list containing all room bookings in the system
#[openapi(tag = "Room Bookings")]
#[get("/room-bookings")]
fn get_room_bookings() -> Json<Vec<RoomBooking>> {
    return Json(storage::fetch_all());
}

/// # Get room bookings for the specified customer id
///
/// Returns a list of bookings.
#[openapi(tag = "Room Bookings")]
#[get("/room-bookings/customer/<customer_id>")]
fn get_customer_room_bookings(customer_id: u32) -> Json<Vec<RoomBooking>> {
    return Json(storage::fetch_by_customer_id(customer_id));
}

/// # Get room bookings for the specified room type
///
/// Returns a list of bookings.
#[openapi(tag = "Room Bookings")]
#[get("/room-bookings/room-type/<room_type_id>")]
fn get_room_type_bookings(room_type_id: u8) -> Json<Vec<RoomBooking>> {
    return Json(storage::fetch_by_room_type_id(room_type_id));
}

#[rocket::main]
async fn main() {
    let launch_result = rocket::build()
        .mount(
            "/",
            openapi_get_routes![
                get_room_booking,
                create_room_booking,
                get_room_bookings,
                get_customer_room_bookings,
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
