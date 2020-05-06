extern crate rusqlite;

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
use rusqlite::params;

// List of schema updates
static SCHEMA_UPDATE_LIST: [&str; 1] = [
        "CREATE TABLE repo (
                url TEXT NOT NULL,
                last_fetch TEXT NOT NULL,
                fetch_status TEXT NOT NULL
                )",
    ];

pub fn init() -> Result<(), rusqlite::Error> {

    // Creates DB file if it doesn't exist.
    let conn = Connection::open("auto-clippy.sqlite3")?;

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
        let result: Result<String> = conn.query_row(
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
                updates_applied = updates_applied + 1;
            },
        }
    }
    if updates_applied > 0 {
        println!("Applied {} database update(s).", updates_applied);
    }

    Ok(())
}
