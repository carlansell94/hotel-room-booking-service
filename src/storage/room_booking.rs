/*
    SPDX-License-Identifier: GPL-3.0-or-later
    SPDX-FileCopyrightText: Copyright © 2023 Carl Ansell <@carlansell94>
*/

use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Defines the allowed values for the status of a booking
#[derive(Clone, Serialize, Deserialize, JsonSchema, PartialEq, Debug)]
pub enum BookingStatus {
    /// A booking that has been paid for, but the user has not yet checked in
    Confirmed,
    /// A booking which has been completed by the user checking in to their room
    Complete,
    /// A booking that the user has cancelled
    Cancelled,
}

impl BookingStatus {
    /// Converts a booking status string into the corresponding BookingStatus enum
    ///
    /// # Arguments
    ///
    /// * `value` - A string containing the value to convert
    ///
    /// # Examples
    ///
    /// ```
    /// let status = BookingStatus::from_string("Complete");
    /// ```
    pub fn from_string(value: &str) -> Option<BookingStatus> {
        match value {
            "Confirmed" => Some(BookingStatus::Confirmed),
            "Complete" => Some(BookingStatus::Complete),
            "Cancelled" => Some(BookingStatus::Cancelled),
            _ => None,
        }
    }
}

/// Describes a single room booking
#[derive(Clone, Serialize, Deserialize, JsonSchema, PartialEq, Debug)]
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
    /// Sets the booking id of the current booking.
    ///
    /// # Arguments
    ///
    /// * `booking_id` - A u32 which holds the id of the booking
    ///
    /// # Examples
    ///
    /// ```
    /// booking.set_booking_id(1);
    /// ```
    pub fn set_booking_id(&mut self, booking_id: u32) {
        self.booking_id = Some(booking_id);
    }

    /// Sets the status of the current booking.
    ///
    /// # Arguments
    ///
    /// * `booking_status` - A BookingStatus enum value
    ///
    /// # Examples
    ///
    /// ```
    /// booking.set_status(BookingStatus::Cancelled);
    ///
    pub fn set_status(&mut self, booking_status: BookingStatus) {
        self.status = Some(booking_status);
    }
}
