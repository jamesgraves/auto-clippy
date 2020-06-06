// extern crate rusqlite;

// use rusqlite::{Connection, Result};
// use rusqlite::NO_PARAMS;
// use rusqlite::params;
use anyhow::Result;

pub fn batch_run(dry_run: bool, jobs: usize, quiet: bool, verbose: u8) -> Result<usize> {
    println!("run {:?} {:?} {:?} {:?}", dry_run, jobs, quiet, verbose);
    Ok(0)
}

