// #[macro_use]
extern crate structopt;

mod database;
mod status;
mod repos;
mod clippy;

use structopt::StructOpt;
use anyhow::Result;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "q", long = "quiet")]
    /// Quiet, only print errors
    quiet: bool,

    #[structopt(short = "v", long = "version")]
    /// Print version number
    version: bool,

    #[structopt(subcommand)]
    /// Subcommands
    subcommand: Option<Cmd>,
}

#[derive(StructOpt, Debug)]
/// Automatically find and fix Rust code issues.
enum Cmd {
    #[structopt(name = "add")]
    /// Add Rust software source repositories for checking.
    Add { urls: Vec<String> },

    #[structopt(name = "status")]
    /// Show check status for all monitored source code repositories.
    Status {
        #[structopt(short = "v")]
        verbose: bool,
    },

    #[structopt(name = "run")]
    /// Start checking the repositories and fixing things.
    Run {
        #[structopt(short = "c", long = "check-only")]
        /// Don't try to fix anything or create pull requests.
        check_only: bool,

        #[structopt(long = "dry-run")]
        /// Don't pull anything, only print what would be done.
        dry_run: bool,

        /// Verbose mode (-v, -vv, -vvv, etc.).
        #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
        verbose: u8,
    },

    #[structopt(name = "remove")]
    /// Remove Rust software source repositories from being checked.
    Remove { 
        #[structopt(short = "p", long = "purge")]
        /// Delete everything associated with the URLs, including previous status.
        purge: bool,

        urls: Vec<String>,
    },
}

fn dispatch_subcommand(opt: Opt) -> Result<usize> {
    if opt.version {
        println!("version");
        Ok(0)
    } else {
        match opt.subcommand {
            Some(cmd) => {
                match cmd {
                    Cmd::Add { urls } => repos::add_urls(&urls),
                    Cmd::Status { verbose } => status::statistics(verbose),
                    Cmd::Remove { purge, urls } => repos::remove_urls(purge, &urls),
                    Cmd::Run { check_only, dry_run, verbose, } => clippy::batch_run(check_only, dry_run, verbose)
                }
            },
            None => status::statistics(false),
        }
    }
}

fn main() {
    database::init().expect("Failed to initialize database");

    let opt = Opt::from_args();
    let quiet = opt.quiet;
    match dispatch_subcommand(opt) {
        Ok(count) => {
            if ! quiet {
                println!("items processed: {}", count);
            }
        }
        Err(err) => {
            println!("error: {:?} ", err);
        }
    }
}
