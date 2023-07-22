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

/// The path used to store a snapshot of the stored booking data.
static SNAPSHOT_PATH: &str = "booking.dat";
/// A lazily initialised HashMap containing the list of bookings held by the system.
static BOOKING_LIST: Lazy<Mutex<HashMap<u32, RoomBooking>>> = Lazy::new(|| {
    let map: HashMap<u32, RoomBooking> = HashMap::new();
    Mutex::new(map)
});

/// Checks whether a storage snapshot exists in the path defined by SNAPSHOT_PATH.
pub fn snapshot_exists() -> bool {
    return metadata(SNAPSHOT_PATH).is_ok();
}

/// Loads the snapshot from the path defined by ```SNAPSHOT_PATH``` into the ```BOOKING_LIST``` HashMap.
pub fn load_snapshot() -> Result<(), Box<dyn std::error::Error>> {
    let mut file_content = Vec::new();
    let mut file: File = File::open(SNAPSHOT_PATH)?;
    file.read_to_end(&mut file_content)?;

    let snapshot: HashMap<u32, RoomBooking> = bincode::deserialize(&file_content)
        .map_err(|error| Box::new(error) as Box<dyn std::error::Error>)?;

    *BOOKING_LIST.lock().unwrap() = snapshot;
    return Ok(());
}

/// Saves a snapshot of the ```BOOKING_LIST``` HashMap to the path defined by ```SNAPSHOT_PATH```.
/// Data is converted to binary for improved storage efficiency.
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

/// Create a new booking.
///
/// # Arguments
///
/// * `booking` - A RoomBooking object containing details of the booking. ```booking_id``` and
/// ```status``` should be excluded as these are added automatically.
///
/// # Examples
///
/// ```
/// booking = RoomBooking {
///     customer_id: 1,
///     room_type_id: 1,
///     check_in_date: "2020-01-01".to_string(),
///     check_out-date: "2020-01-08".to_string()
/// }
///
/// create(booking);
/// ```
pub fn create(mut booking: RoomBooking) -> Result<RoomBooking, ()> {
    if booking.booking_id != None || booking.status != None {
        return Err(());
    }

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

/// Update the status of a booking.
///
/// # Arguments
///
/// * `booking_id` - The id of the booking to update
/// * `status` - The BookingStatus enum to be applied to the booking
///
/// # Examples
///
/// ```
/// status(1, BookingStatus::Complete);
/// ```
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

/// Fetch a booking using a booking id.
///
/// # Arguments
///
/// * `booking_id` - The booking id of the booking to return.
///
/// # Examples
///
/// ```
/// booking = fetch_by_id(1);
/// ```
pub fn fetch_by_id(booking_id: u32) -> Option<RoomBooking> {
    let booking_list: std::sync::MutexGuard<'_, HashMap<u32, RoomBooking>> =
        match BOOKING_LIST.lock() {
            Ok(guard) => guard,
            Err(_) => return None,
        };

    let result: Option<RoomBooking> = booking_list.get(&booking_id).cloned();
    return result;
}

/// Fetch a list of bookings made by a specific customer.
///
/// # Arguments
///
/// * `customer_id` - The customer id of the bookings to return.
///
/// # Examples
///
/// ```
/// bookings = fetch_by_customer_id(1);
/// ```
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

/// Fetch a list of bookings with a specific check in date.
///
/// # Arguments
///
/// * `date` - A string containing the check in date of the bookings to return.
///
/// # Examples
///
/// ```
/// bookings = fetch_by_check_in_date("2020-01-01".to_string());
/// ```
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

/// Fetch a list of bookings made by a specific customer.
///
/// # Arguments
///
/// * `customer_id` - The customer id of the bookings to return.
///
/// # Examples
///
/// ```
/// bookings = fetch_by_customer_id(1);
/// ```
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

/// Fetch a list of all room bookings.
///
/// # Examples
///
/// ```
/// bookings = fetch_all();
/// ```
pub fn fetch_all() -> Vec<RoomBooking> {
    let list: std::sync::MutexGuard<'_, HashMap<u32, RoomBooking>> = match BOOKING_LIST.lock() {
        Ok(guard) => guard,
        Err(_) => {
            return Vec::new();
        }
    };

    return list.values().cloned().collect();
}

#[cfg(test)]
mod tests {
    use super::room_booking::RoomBooking;
    use crate::storage::*;

    /// Describes a single room booking
    fn dummmy_booking() -> RoomBooking {
        return RoomBooking {
            booking_id: None,
            customer_id: 1,
            room_type_id: 3,
            check_in_date: "2020-01-01".to_string(),
            check_out_date: "2020-01-08".to_string(),
            status: None,
        };
    }

    /// Describes the expected output when the dummy_booking is created
    fn dummmy_booking_success() -> RoomBooking {
        return RoomBooking {
            booking_id: Some(1),
            customer_id: 1,
            room_type_id: 3,
            check_in_date: "2020-01-01".to_string(),
            check_out_date: "2020-01-08".to_string(),
            status: Some(BookingStatus::Confirmed),
        };
    }

    #[test]
    fn create_booking() {
        assert_eq!(create(dummmy_booking()), Ok(dummmy_booking_success()));

        let failed_booking = RoomBooking {
            booking_id: Some(5),
            customer_id: 4,
            room_type_id: 2,
            check_in_date: "2020-01-01".to_string(),
            check_out_date: "2020-01-08".to_string(),
            status: None,
        };

        assert!(create(failed_booking).is_err());
    }

    #[test]
    fn fetch_booking() {
        // Ensure a booking exists before continuing tests.
        while let None = fetch_by_id(1) {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        let booking: RoomBooking = fetch_by_id(1).unwrap();
        assert_eq!(booking, dummmy_booking_success());
    }

    #[test]
    fn update_booking_status() {
        // Wait for a booking to exist before continuing. Ensures we create a booking with
        // the expected id for this test.
        while let None = fetch_by_id(1) {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        assert!(create(dummmy_booking()).is_ok());

        assert_eq!(status(2, BookingStatus::Complete), true);
        let booking: RoomBooking = fetch_by_id(2).unwrap();
        assert_eq!(booking.status, Some(BookingStatus::Complete));
    }
}
