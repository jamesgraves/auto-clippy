// extern crate rusqlite;

// use rusqlite::{Connection, Result};
// use rusqlite::NO_PARAMS;
// use rusqlite::params;
use super::error::RuntimeError;

fn verbose_statistics() -> Result<(), RuntimeError> {
    println!("verbose statistics");
    Ok(())
}

pub fn statistics(verbose: bool) -> Result<usize, RuntimeError> {
    if verbose {
        verbose_statistics()?;
    }
    println!("statistics");
    Ok(0)
}

