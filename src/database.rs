extern crate rusqlite;

use rusqlite::{Connection, NO_PARAMS, params};

use anyhow::{Result, anyhow};

// List of schema updates
static SCHEMA_UPDATE_LIST: [&str; 1] = [
        "CREATE TABLE repo (
                url TEXT UNIQUE NOT NULL,
                last_fetch TEXT NOT NULL,
                fetch_status TEXT NOT NULL
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
            Err(_err) => {
                conn.execute(update, NO_PARAMS)?;
                conn.execute("INSERT INTO schema_updates (schema_update) VALUES (?1)",
                    params![update])?;
                updates_applied += 1;
            },
        }
    }
    if updates_applied > 0 {
        println!("Applied {} database update(s).", updates_applied);
    }

    Ok(())
}

// Set the current status for a repo.
// Inserts new repo record if needed.
pub fn set_repo_status(url: &str, status: &str) -> Result<()> {
    let conn = open()?;

    let update_count = conn.execute("UPDATE repo SET fetch_status = ?2 WHERE url = ?1",
                              params![url, status])?;
    if update_count == 1 {
        return Ok(()); // found it, updated
    }

    // url was not found.
    let insert_result = conn.execute("INSERT INTO repo (url, last_fetch, fetch_status) VALUES (?1, '', ?2)",
                              params![url, status])?;
    if insert_result == 1 {
        return Ok(()); // found it, updated
    }

    Err(anyhow!("unable to update url table"))
}

