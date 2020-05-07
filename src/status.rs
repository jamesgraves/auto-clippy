// extern crate rusqlite;

// use rusqlite::{Connection, Result};
// use rusqlite::NO_PARAMS;
// use rusqlite::params;
use anyhow::Result;

fn verbose_statistics() -> Result<()> {
    println!("verbose statistics");
    Ok(())
}

pub fn statistics(verbose: bool) -> Result<usize> {
    if verbose {
        verbose_statistics()?;
    }
    println!("statistics");
    Ok(0)
}

