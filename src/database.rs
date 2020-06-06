extern crate rusqlite;

use rusqlite::{Connection, NO_PARAMS, params};

use anyhow::{Result, anyhow};

// statuses:
//
// new
// disabled
// error: fetch_failed - last fetch failed for some reason.

// List of schema updates
static SCHEMA_UPDATE_LIST: [&str; 1] = [
        "CREATE TABLE repo (
                url TEXT UNIQUE NOT NULL,
                status TEXT NOT NULL,
                reference_count INTEGER NOT NULL
                )",
    ];

// Open a connection to the database.
pub fn open() -> Result<rusqlite::Connection> {
    let conn = Connection::open("auto-clippy.sqlite3")?;
    Ok(conn)
}

// Create the database file, and update the schema to bring it up to
// the current version.
pub fn init() -> Result<()> {

    // Creates DB file if it doesn't exist.
    let conn = open()?;

    // TODO: transactions

    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_updates (
            schema_update STRING PRIMARY KEY NOT NULL UNIQUE
            )",
            NO_PARAMS,
        )?;

    let mut updates_applied = 0;
    for update in SCHEMA_UPDATE_LIST.iter() {
        // if this update exists in schema_updates, skip
        // otherwise run it to update on-disk database schema to match latest version.
        let result: rusqlite::Result<String> = conn.query_row(
            "SELECT schema_update FROM schema_updates WHERE schema_update = ?1",
            params![update],
            |row| row.get(0),
            );
        match result {
            Ok(_found_update) => continue,
            Err(err) => {
                match err {
                    rusqlite::Error::QueryReturnedNoRows => {
                        conn.execute(update, NO_PARAMS)?;
                        conn.execute("INSERT INTO schema_updates (schema_update) VALUES (?1)",
                        params![update])?;
                        updates_applied += 1;
                    },
                    _ => return Err(anyhow!("database error")),
                }
            },
        }
    }
    if updates_applied > 0 {
        println!("Applied {} database update(s).", updates_applied);
    }

    Ok(())
}

// Add repo with given reference count.
pub fn add_repo(url: &str, reference_count: isize) -> Result<()> {
    let conn = open()?;

    let insert_result = conn.execute("INSERT INTO repo (url, status, reference_count) VALUES (?1, 'new', ?2)",
                              params![url, reference_count])?;
    if insert_result == 1 {
        Ok(())
    } else {
        Err(anyhow!(format!("failed to insert url: {} with ref count: {}", url, reference_count)))
    }
}

// Delete a repo.
pub fn delete_repo(url: &str) -> Result<()> {
    let conn = open()?;

    let remove_count = conn.execute("DELETE FROM repo WHERE url = ?1",
                              params![url])?;
    if remove_count == 1 {
        Ok(())
    } else {
        Err(anyhow!(format!("failed to delete url: {}", url)))
    }
}


pub fn url_exists(url: &str) -> Result<bool> {
    let conn = open()?;

    let result: rusqlite::Result<i64> = conn.query_row(
        "SELECT 1 FROM repo WHERE url = ?1",
        params![url],
        |row| row.get(0),
        );

    match result {
        Ok(_found) => Ok(true),
        Err(err) => {
            match err {
                rusqlite::Error::QueryReturnedNoRows => Ok(false),
                _ => Err(anyhow!("database error")),
            }
        }
    }
}


// Set the current status for a repo.
pub fn set_repo_status(url: &str, status: &str) -> Result<()> {
    let conn = open()?;

    let update_count = conn.execute("UPDATE repo SET status = ?2 WHERE url = ?1",
                              params![url, status])?;
    if update_count == 1 {
        return Ok(());
    } else {
        return Err(anyhow!(format!("failed to update url: {} with status: {}", url, status)));
    }
}

/*
// Adjust reference count for a URL.
pub fn adjust_reference_count(url: &str, adjustment: isize) -> Result<isize> {
    let conn = open()?;
    let tx = conn.transaction()?;

    let mut curr_rc: i64;

    // shorten?
    let result: rusqlite::Result<i64> = tx.query_row(
        "SELECT reference_count FROM repo WHERE url = ?1",
        params![url],
        |row| row.get(0),
        );
    match result {
        Ok(rc) => curr_rc = rc,
        Err(err) => return Err(anyhow!(format!("failed to get ref count for: {}", url))),
    }

    curr_rc += adjustment;
    if curr_rc < 0 {
        curr_rc = 0;
    }

    let update_count = conn.execute("UPDATE repo SET reference_count = ?2 WHERE url = ?1",
                              params![url, curr_rc])?;
    tx.commit();
    if update_count == 1 {
        return Ok(());
    } else {
        return Err(anyhow!(format!("failed to update url: {} with ref count adjustment: {}", url, adjustment)));
    }
}
*/


