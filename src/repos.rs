use super::database;
use anyhow::{Result, anyhow};

static USER_ADDED_REFCOUNT: isize = 1000000;
static REPOS_DIR: &str = "repos";

/*
static COMMON_HOSTING_SITES: [&str; 6] = [
    "beanstalkapp.com",
    "bitbucket.org",
    "github.com",
    "gitlab.com",
    "launchpad.net",
    "sourceforge.net",
];
*/

/*
fn add_https(repo: String) -> String{
    for hosting_site in COMMON_HOSTING_SITES.iter() {
        if repo.starts_with(hosting_site) {
            repo.insert_str(0, "https://");
        }
    }
}
*/

/// Skips http:// or https:// from url.
fn skip_http_https(repo: &str) -> &str {
    if repo.starts_with("http://") {
        return &repo[6..];
    }
    if repo.starts_with("https://") {
        return &repo[7..];
    }
    repo
}

pub fn add_urls(recursive: usize, urls: &[String]) -> Result<usize> {
    _add_urls(recursive, urls, USER_ADDED_REFCOUNT)
}

// Internal version with reference count.
fn _add_urls(recursive: usize, urls: &[String], reference_count: isize) -> Result<usize> {
    let mut count: usize = 0;
    println!("recursive: {}", recursive);
    if urls.is_empty() {
        return Err(anyhow!("one or more URLs required"))
    }

    for url in urls.iter() {
        let url = skip_http_https(url);
        if ! database::url_exists(url)? {
            database::add_repo(url, reference_count)?;
            database::set_repo_status(url, "new")?;
            count += 1;
            println!("added: {}", url);
        }

        // let refcount = database::adjust_reference_count(url, reference_count)?;
        if recursive > 0 {
            // generate list of dependent crates
            // count += _add_urls(recursive - 1, new_urls, 1)?;
        }
    }

    Ok(count)
}

pub fn disable_urls(recursive: usize, urls: &[String]) -> Result<usize> {
    _disable_urls(recursive, urls, -1 * USER_ADDED_REFCOUNT)
}

fn _disable_urls(recursive: usize, urls: &[String], _reference_count: isize) -> Result<usize> {
    let mut count: usize = 0;
    println!("recursive: {}", recursive);
    if urls.is_empty() {
        return Err(anyhow!("one or more URLs required"))
    }

    for url in urls.iter() {
        let url = skip_http_https(url);
        if database::url_exists(url)? {
            database::set_repo_status(url, "disabled")?;
            count += 1;
            println!("disabled: {}", url);
            // generate list of dependent crates
            // count += _add_urls(recursive - 1, new_urls, -1)?;
        }
    }

    Ok(count)
}


pub fn remove_urls(recursive: usize, urls: &[String]) -> Result<usize> {
    let mut count: usize = 0;
    println!("recursive: {}", recursive);
    if urls.is_empty() {
        return Err(anyhow!("one or more URLs required"))
    }

    for url in urls.iter() {
        let url = skip_http_https(url);
        if database::url_exists(url)? {
            database::delete_repo(url)?;
            count += 1;
            println!("removed: {}", url);
            // TODO: remove on-disk
            // generate list of dependent crates
        }
    }

    Ok(count)
}


pub fn init_repos_dir() -> Result<()> {
    use std::fs;

    match fs::metadata(REPOS_DIR) {
        Ok(metadata) => if metadata.is_dir() {
            return Ok(());
        } else {
            return Err(anyhow!("repos exists but is not a directory."));
        },
        Err(_err) => {
            fs::create_dir(REPOS_DIR)?;
        },
    }

    Ok(())
}


// TODO: Use OS independent path manipulation.
/*
fn setup_repo_dir(url: &str) -> Result<&str> {
    let last_dir_sep = url.rfind("/");
    if let Some(last_dir_sep_idx) = last_dir_sep {
        let base_dir, _ = url.split_at(last_dir_sep_idx);
    }

    Ok(&url[..0])
}
*/
