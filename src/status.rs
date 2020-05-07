// extern crate rusqlite;

// use rusqlite::{Connection, Result};
// use rusqlite::NO_PARAMS;
// use rusqlite::params;
use super::error::RuntimeError;

pub fn statistics(verbose: bool) -> Result<usize, RuntimeError> {
    println!("statistics, verbose: {}", verbose);
    Ok(0)
}

