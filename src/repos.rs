use super::database;
use anyhow::{Result, anyhow};
use std::string::String;
use git2::Repository;

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
        return &repo[7..];
    }
    if repo.starts_with("https://") {
        return &repo[8..];
    }
    repo
}

fn elide_dot_git(repo: &str) -> &str {
    if repo.ends_with(".git") {
        let len = repo.len();
        return &repo[..(len - 4)];
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
        let url = elide_dot_git(url);
        if ! database::url_exists(url)? {
            database::add_repo(url, reference_count)?;
            database::set_repo_status(url, "new")?;
            count += 1;
            if let Some(idx) = url.rfind("/") {
                let (repo_parent_dir, _) = url.split_at(idx);
                let (_, repo_name) = url.split_at(idx + 1);
                // TODO: Use OS independent path manipulation.
                let repo_parent_dir = format!("{}/{}", REPOS_DIR, &repo_parent_dir);
                std::fs::create_dir_all(&repo_parent_dir)?;
                let full_url = format!("https://{}.git", url);
                let repo = Repository::clone(&full_url, &repo_parent_dir)?;
                println!("added and cloned: {}    {}", repo_parent_dir, repo_name);
                if repo.is_bare() {
                    println!("bare?");
                }
            }

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
    std::fs::create_dir(REPOS_DIR)?;
    Ok(())
}



/*
fn setup_repo_dir(url: &str) -> Result<&str> {
    let last_dir_sep = url.rfind("/");
    if let Some(last_dir_sep_idx) = last_dir_sep {
        let base_dir, _ = url.split_at(last_dir_sep_idx);
    }

    Ok(&url[..0])
}
*/
