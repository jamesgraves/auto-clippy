// extern crate rusqlite;

// use rusqlite::{Connection, Result};
// use rusqlite::NO_PARAMS;
// use rusqlite::params;
use anyhow::{Result, anyhow};

static COMMON_HOSTING_SITES: [&str; 3] = [
    "github.com",
    "gitlab.com",
    "bitbucket.com",
];

fn canonicalize_repo(repo: &mut String) {
    for hosting_site in COMMON_HOSTING_SITES.iter() {
        if repo.starts_with(hosting_site) {
            repo.insert_str(0, "https://");
        }
    }
}

pub fn add_urls(urls: &[String]) -> Result<usize> {
    if urls.is_empty() {
        return Err(anyhow!("one or more URLs required"))
    }

    let mut m_urls = urls.to_owned();
    for url in m_urls.iter_mut() {
        canonicalize_repo(url);
        println!("url {}", url);
    }

    Ok(urls.len())
}

pub fn remove_urls(purge: bool, urls: &[String]) -> Result<usize> {
    if urls.is_empty() {
        return Err(anyhow!("one or more URLs required"))
    }

    let mut m_urls = urls.to_owned();
    for url in m_urls.iter_mut() {
        canonicalize_repo(url);
        println!("url {}", url);
    }

    if purge {
        println!("also purge");
    }
    Ok(urls.len())
}

