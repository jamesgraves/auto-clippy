// #[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "auto-clippy")]
/// Automatically find and fix Rust code issues.
enum Opt {
    #[structopt(name = "init")]
    /// Initialize local database and check prerequisites.
    Init {
        #[structopt(short = "d", default_value = "sqlite")]
        database: String,
    },

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

        #[structopt(short = "q", long = "quiet")]
        /// Don't print anything except errors.
        quiet: bool,
    },

    #[structopt(name = "remove")]
    /// Remove Rust software source repositories from being checked.
    Remove { urls: Vec<String> },

    #[structopt(name = "purge")]
    /// Delete all data associated with with a repository.
    Purge { urls: Vec<String> },
}

#[derive(Debug)]
pub enum RuntimeError {
    NetworkError,
    LocalError,
}

fn dispatch_subcommand(opt: Opt) -> Result<usize, RuntimeError> {
    match opt {
        Opt::Init { database } => println!("init {:?}", database),
        Opt::Add { urls } => println!("add {:?}", urls),
        Opt::Status { verbose } => println!("status {:?}", verbose),
        Opt::Run {
            check_only,
            dry_run,
            verbose,
            quiet,
        } => println!(
            "run {:?} {:?} {:?} {:?}",
            check_only, dry_run, verbose, quiet
        ),
        Opt::Remove { urls } => println!("remove {:?}", urls),
        Opt::Purge { urls } => println!("purge {:?}", urls),
    }

    Ok(0)
}

fn main() {
    let opt = Opt::from_args();
    let cmd_result = dispatch_subcommand(opt);
    match cmd_result {
        Ok(count) => {
            println!("items processed: {}", count);
        }
        Err(err) => {
            println!("error: {:?} ", err);
        }
    }
}
