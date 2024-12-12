use std::collections::HashMap;
use chrono::Local;
use sled::Db;

/// check_and_reset_counter checks if the counters need to be reset and resets them if needed
pub fn check_and_reset_counter(db: &Db) {
    // Key used to store the last reset date in the database
    let date_key = "last_reset_date";

    // Get the current date (formatted as YYYY-MM-DD)
    let current_date = Local::now().format("%Y-%m-%d").to_string();

    // Retrieve the stored date from the database
    let stored_date = db.get(date_key).ok().flatten();

    if let Some(stored_date) = stored_date {
        // Convert the stored date from bytes to a string
        let stored_date = String::from_utf8(stored_date.to_vec()).unwrap();

        if stored_date != current_date {
            // If the dates don't match, reset counters and update the date
            println!(
                "Resetting counters. Previous date: {}, Current date: {}",
                stored_date, current_date
            );
            reset_counter(db);
            db.insert(date_key, current_date.as_bytes())
                .expect("Failed to update reset date");
        } else {
            // No reset needed, dates match
            println!("No reset needed. Current date matches stored date.");
        }
    } else {
        // If no date is stored, assume first run and initialize the date
        println!("No stored date found. Initializing counters and storing today's date.");
        reset_counter(db);
        db.insert(date_key, current_date.as_bytes())
            .expect("Failed to initialize reset date");
    }
}

// increment_counter increments the counter for a specific route, stop, hour, and minute
pub fn increment_counter(db: &Db, key: String, qty: i32) -> i32 {
    let current_value = db
        .get(&key)
        .ok()
        .flatten()
        .map(|v| {
            String::from_utf8(v.to_vec())
                .unwrap()
                .parse::<i32>()
                .unwrap_or(0)
        })
        .unwrap_or(0);

    let new_value = current_value + qty;
    db.insert(key, new_value.to_string().as_bytes())
        .expect("Failed to increment counter");
    new_value
}

// reset_counter resets all counters
pub fn reset_counter(db: &Db) {
    db.clear().expect("Failed to reset counters");
}

// get_counter retrieves the counter for a specific route, stop, hour, and minute
pub fn get_stop_counters(db: &Db, key: String) -> HashMap<String, i32> {
    db.scan_prefix(&key)
        .filter_map(|item| {
            item.ok().and_then(|(raw_key, raw_value)| {
                let key = String::from_utf8(raw_key.to_vec()).ok()?;
                let value = String::from_utf8(raw_value.to_vec()).ok()?.parse::<i32>().ok()?;
                Some((key, value))
            })
        })
        .collect()
}

// bulk_get_counter retrieves the counters for multiple keys with the same prefix
pub fn bulk_get_counter(db: &Db, keys: Vec<String>) -> HashMap<String, i32> {
    keys.into_iter()
        .map(|prefix| {
            let total_count = db
                .scan_prefix(&prefix)
                .filter_map(|item| {
                    item.ok()
                        .and_then(|(_, value)| String::from_utf8(value.to_vec()).ok()?.parse::<i32>().ok())
                })
                .sum();
            (prefix, total_count)
        })
        .collect()
}