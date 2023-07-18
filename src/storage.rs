/*
    SPDX-License-Identifier: GPL-3.0-or-later
    SPDX-FileCopyrightText: Copyright © 2023 Carl Ansell <@carlansell94>
*/

use self::room_booking::{BookingStatus, RoomBooking};
use once_cell::sync::Lazy;
use std::fs::{metadata, File};
use std::io::{Read, Write};
use std::{collections::HashMap, sync::Mutex};
pub mod room_booking;

static SNAPSHOT_PATH: &str = "booking.dat";
static BOOKING_LIST: Lazy<Mutex<HashMap<u32, RoomBooking>>> = Lazy::new(|| {
    let map: HashMap<u32, RoomBooking> = HashMap::new();
    Mutex::new(map)
});

pub fn snapshot_exists() -> bool {
    return metadata(SNAPSHOT_PATH).is_ok();
}

pub fn load_snapshot() -> Result<(), Box<dyn std::error::Error>> {
    let mut file_content = Vec::new();
    let mut file: File = File::open(SNAPSHOT_PATH)?;
    file.read_to_end(&mut file_content)?;

    let snapshot: HashMap<u32, RoomBooking> = bincode::deserialize(&file_content)
        .map_err(|error| Box::new(error) as Box<dyn std::error::Error>)?;

    *BOOKING_LIST.lock().unwrap() = snapshot;
    return Ok(());
}

fn save_snapshot(booking_list: &HashMap<u32, RoomBooking>) -> bool {
    let snapshot: Vec<u8> = bincode::serialize(&booking_list).unwrap_or_else(|_| {
        return Vec::new();
    });

    let mut file = match File::create(SNAPSHOT_PATH) {
        Ok(file) => file,
        Err(_) => {
            return false;
        }
    };

    match file.write_all(&snapshot) {
        Ok(_) => return true,
        Err(_) => return false,
    };
}

pub fn create(mut booking: RoomBooking) -> Result<RoomBooking, ()> {
    let mut booking_list: std::sync::MutexGuard<'_, HashMap<u32, RoomBooking>> =
        match BOOKING_LIST.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(()),
        };
    let max_id = booking_list.keys().fold(std::u32::MIN, |a, b| a.max(*b));
    let next_id = max_id + 1;
    booking.set_booking_id(next_id);
    booking.set_status(BookingStatus::Confirmed);
    booking_list.insert(next_id, booking.clone());
    save_snapshot(&*booking_list);
    return Ok(booking);
}

pub fn status(booking_id: u32, status: BookingStatus) -> bool {
    let mut booking_list: std::sync::MutexGuard<'_, HashMap<u32, RoomBooking>> =
        match BOOKING_LIST.lock() {
            Ok(guard) => guard,
            Err(_) => return false,
        };

    let booking: &mut RoomBooking = match booking_list.get_mut(&booking_id) {
        Some(booking) => booking,
        None => return false,
    };

    if booking.status != Some(BookingStatus::Confirmed) {
        return false;
    }

    booking.set_status(status);
    save_snapshot(&*booking_list);
    return true;
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

pub fn fetch_by_check_in_date(date: &str) -> Vec<RoomBooking> {
    let booking_list: std::sync::MutexGuard<'_, HashMap<u32, RoomBooking>> =
        match BOOKING_LIST.lock() {
            Ok(guard) => guard,
            Err(_) => return Vec::new(),
        };

    let results: Vec<RoomBooking> = booking_list
        .values()
        .filter(|booking: &&RoomBooking| booking.check_in_date == date)
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
