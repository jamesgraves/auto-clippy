// extern crate rusqlite;

use rusqlite::{Connection, NO_PARAMS};
// use rusqlite::params;
use anyhow::Result;
use super::database;

fn verbose_statistics(_conn: &Connection) -> Result<()> {
    Ok(())
}

pub fn statistics(verbose: bool) -> Result<usize> {

    let conn = database::open()?;

    if verbose {
        verbose_statistics(&conn)?;
    }

    let count: isize =  conn.query_row(
        "SELECT count(url) FROM repo",
        NO_PARAMS,
        |row| row.get(0),
        )?;
    println!("project count: {}", count);
    Ok(0)
}

