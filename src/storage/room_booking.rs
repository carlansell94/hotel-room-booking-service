/*
    SPDX-License-Identifier: GPL-3.0-or-later
    SPDX-FileCopyrightText: Copyright © 2023 Carl Ansell <@carlansell94>
*/

use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub enum BookingStatus {
    Confirmed,
    Complete,
    Cancelled,
}

impl BookingStatus {
    pub fn from_string(value: &str) -> Option<BookingStatus> {
        match value {
            "Confirmed" => Some(BookingStatus::Confirmed),
            "Complete" => Some(BookingStatus::Complete),
            "Cancelled" => Some(BookingStatus::Cancelled),
            _ => None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoomBooking {
    pub booking_id: Option<u32>,
    pub customer_id: u32,
    pub room_type_id: u8,
    pub check_in_date: String,
    pub check_out_date: String,
    pub status: Option<BookingStatus>,
}

impl RoomBooking {
    pub fn set_booking_id(&mut self, booking_id: u32) {
        self.booking_id = Some(booking_id);
    }

    pub fn set_status(&mut self, booking_status: BookingStatus) {
        self.status = Some(booking_status);
    }
}
