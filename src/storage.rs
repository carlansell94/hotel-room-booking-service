/*
    SPDX-License-Identifier: GPL-3.0-or-later
    SPDX-FileCopyrightText: Copyright © 2023 Carl Ansell <@carlansell94>
*/

use self::room_booking::RoomBooking;
use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};
pub mod room_booking;

static BOOKING_LIST: Lazy<Mutex<HashMap<u32, RoomBooking>>> = Lazy::new(|| {
    let map: HashMap<u32, RoomBooking> = HashMap::new();
    Mutex::new(map)
});

pub fn create(mut booking: RoomBooking) -> Result<RoomBooking, ()> {
    let mut booking_list: std::sync::MutexGuard<'_, HashMap<u32, RoomBooking>> =
        match BOOKING_LIST.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(()),
        };
    let max_id = booking_list.keys().fold(std::u32::MIN, |a, b| a.max(*b));
    let next_id = max_id + 1;
    booking.set_booking_id(next_id);
    booking_list.insert(next_id, booking.clone());

    return Ok(booking);
}

pub fn fetch_booking(booking_id: u32) -> Option<RoomBooking> {
    let booking_list: std::sync::MutexGuard<'_, HashMap<u32, RoomBooking>> =
        match BOOKING_LIST.lock() {
            Ok(guard) => guard,
            Err(_) => return None,
        };

    let result: Option<RoomBooking> = booking_list.get(&booking_id).cloned();
    return result;
}

pub fn fetch_by_customer_id(customer_id: u32) -> Vec<RoomBooking> {
    let booking_list: std::sync::MutexGuard<'_, HashMap<u32, RoomBooking>> =
        match BOOKING_LIST.lock() {
            Ok(guard) => guard,
            Err(_) => return Vec::new(),
        };

    let results: Vec<RoomBooking> = booking_list
        .values()
        .filter(|booking: &&RoomBooking| booking.customer_id == customer_id)
        .cloned()
        .collect();

    results
}

pub fn fetch_by_room_type_id(room_type_id: u8) -> Vec<RoomBooking> {
    let booking_list: std::sync::MutexGuard<'_, HashMap<u32, RoomBooking>> =
        match BOOKING_LIST.lock() {
            Ok(guard) => guard,
            Err(_) => return Vec::new(),
        };

    let results: Vec<RoomBooking> = booking_list
        .values()
        .filter(|booking: &&RoomBooking| booking.room_type_id == room_type_id)
        .cloned()
        .collect();

    results
}

pub fn fetch_all() -> Vec<RoomBooking> {
    let list: std::sync::MutexGuard<'_, HashMap<u32, RoomBooking>> = match BOOKING_LIST.lock() {
        Ok(guard) => guard,
        Err(_) => {
            return Vec::new();
        }
    };

    return list.values().cloned().collect();
}
