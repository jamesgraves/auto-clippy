// extern crate rusqlite;

// use rusqlite::{Connection, Result};
// use rusqlite::NO_PARAMS;
// use rusqlite::params;
use anyhow::Result;

pub fn batch_run(check_only: bool, dry_run: bool, verbose: u8) -> Result<usize> {
    println!("run {:?} {:?} {:?}", check_only, dry_run, verbose);
    Ok(0)
}

