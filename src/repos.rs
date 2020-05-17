use super::database;
use anyhow::{Result, anyhow};

static COMMON_HOSTING_SITES: [&str; 6] = [
    "beanstalkapp.com",
    "bitbucket.org",
    "github.com",
    "gitlab.com",
    "launchpad.net",
    "sourceforge.net",
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
        database::set_repo_status(url, "new", true)?;
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
        if purge {
            database::delete_repo(url)?;
        } else {
            database::set_repo_status(url, "disabled", false)?;
        }
        println!("url {}", url);
    }

    Ok(urls.len())
}

