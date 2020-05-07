// extern crate rusqlite;

// use rusqlite::{Connection, Result};
// use rusqlite::NO_PARAMS;
// use rusqlite::params;
use super::error::RuntimeError;
use super::error::RuntimeError::InvalidArgument;


pub fn add_urls(urls: &[String]) -> Result<usize, RuntimeError> {
    if urls.is_empty() {
        return Err(InvalidArgument)
    }
    println!("add {:?}", urls);
    Ok(0)
}

pub fn remove_urls(purge: bool, urls: &[String]) -> Result<usize, RuntimeError> {
    if urls.is_empty() {
        return Err(InvalidArgument)
    }
    if purge {
        println!("also purge");
    }
    println!("remove {:?}", urls);
    Ok(0)
}

