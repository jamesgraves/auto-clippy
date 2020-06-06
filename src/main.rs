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

    /// Print version number
    #[structopt(short, long)]
    version: bool,

    /// Subcommands
    #[structopt(subcommand)]
    subcommand: Option<Cmd>,
}

/// Automatically find and fix Rust code issues.
#[derive(StructOpt, Debug)]
enum Cmd {

    /// Add Rust software source repositories for checking.
    #[structopt(name = "add")]
    Add {

        /// Recursively add library dependencies up to depth N, 0 means none.
        #[structopt(short, long, default_value = "0")]
        recursive: usize,

        urls: Vec<String>,
    },

    /// Show check status for all monitored source code repositories.
    #[structopt(name = "status")]
    Status {

        /// Increased detail level
        #[structopt(short, long)]
        detail: bool,
    },

    /// Start checking the repositories and fixing things.
    #[structopt(name = "run")]
    Run {

        /// Don't pull anything, only print what would be done.
        #[structopt(long)]
        dry_run: bool,

        /// Run N jobs in parallel, default is number of cores.
        #[structopt(short, long, default_value = "0")]
        jobs: usize,

        /// Quiet, only print errors
        #[structopt(short, long)]
        quiet: bool,

        /// Verbose mode, repeat for increasing detail (-v, -vv, -vvv).
        #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
        verbose: u8,
    },

    /// Don't run checks on previously added repos.
    #[structopt(name = "disable")]
    Disable {

        /// Recursively disable library dependencies up to depth N, 0 means none.
        #[structopt(short, long, default_value = "0")]
        recursive: usize,

        urls: Vec<String>,
    },

    /// Remove Rust software source repositories from local storage.
    #[structopt(name = "remove")]
    Remove {

        /// Recursively remove library dependencies up to depth N, 0 means none.
        #[structopt(short, long, default_value = "0")]
        recursive: usize,

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
                    Cmd::Add { recursive, urls } => repos::add_urls(recursive, &urls),
                    Cmd::Status { detail } => status::statistics(detail),
                    Cmd::Disable { recursive, urls } => repos::disable_urls(recursive, &urls),
                    Cmd::Remove { recursive, urls } => repos::remove_urls(recursive, &urls),
                    Cmd::Run { dry_run, jobs, quiet, verbose } => clippy::batch_run(dry_run, jobs, quiet, verbose),
                }
            },
            None => status::statistics(false),
        }
    }
}

fn main() {
    database::init().expect("Failed to initialize database");

    let opt = Opt::from_args();
    match dispatch_subcommand(opt) {
        Ok(count) => {
            // TODO: quiet
            println!("repos processed: {}", count);
        },
        Err(err) => {
            eprintln!("error: {:?} ", err);
            std::process::exit(1);
        }
    }
}
