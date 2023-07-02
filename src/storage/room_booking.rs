/*
    SPDX-License-Identifier: GPL-3.0-or-later
    SPDX-FileCopyrightText: Copyright © 2023 Carl Ansell <@carlansell94>
*/

use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoomBooking {
    pub booking_id: Option<u32>,
    pub customer_id: u32,
    pub room_type_id: u8,
    pub check_in_date: String,
    pub check_out_date: String,
}

impl RoomBooking {
    pub fn set_booking_id(&mut self, booking_id: u32) {
        self.booking_id = Some(booking_id);
    }
}
