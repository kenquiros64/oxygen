use sled::Db;

// initialize_counter_db initializes the sled database
pub fn initialize_counter_db() -> Db {
    sled::open("db/counter_db").unwrap()
}