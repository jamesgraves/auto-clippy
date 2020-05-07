// extern crate rusqlite;

// use rusqlite::{Connection, Result};
// use rusqlite::NO_PARAMS;
// use rusqlite::params;
use anyhow::{Result, anyhow};


pub fn add_urls(urls: &[String]) -> Result<usize> {
    if urls.is_empty() {
        return Err(anyhow!("one or more URLs required"))
    }
    println!("add {:?}", urls);
    Ok(urls.len())
}

pub fn remove_urls(purge: bool, urls: &[String]) -> Result<usize> {
    if urls.is_empty() {
        return Err(anyhow!("one or more URLs required"))
    }
    if purge {
        println!("also purge");
    }
    println!("remove {:?}", urls);
    Ok(urls.len())
}

