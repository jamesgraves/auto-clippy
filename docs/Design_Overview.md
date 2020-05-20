Design Overview
===============

This program is designed to improve the entire Rust programming language
software ecosystem in as automated a fashion as possible.


# Planned Features

* Clone software repos, and pull updates when available.
* Scan source code using Rust's `clippy` and other tools.
    * Update database on new problems found, and old problems that have disappeared.
* Run spelling and grammar checks on documentation, for human review.
* Help user create local branch, pull request, etc.
* Track work in a database (defaults to local sqlite3)
    * When re-run, check which repos need to have changes pulled, and tests re-run.
    * For example: If the clippy version hasn't changed, and the software repo hasn't been updated, it isn't necessary to run clippy again on this repo.
    * Track pull request status (github). Delete local branches when PRs are accepted.

Future Improvements
===================

* Help create fixes for problems found.
    * Use `rustfix`?
    * Create new appropriately-named branch.
    * (Disabled by default) Automatically create pull requests (github).
* Support all source code sharing and CI sites (gitlab, bitbucket, etc.).
    * *All* popular crates have source hosted on github at the time of writing.
    * Support non-github pull requests equivalents.
* Allow multiple instances to be run in parallel.
    * All work status updates managed through the database.
    * Support distributed databases for even greater scaling.
* Use ML trained on existing bug fixes to help identify issues in code.
* Use ML to suggest a summary comment for functions that don't have any comments.
    * What else can be done to improve documentation?
* Run `cargo audit` to scan for known vulnerabilities.
* Can `cargo geiger` be used to look for unnecessary uses of `unsafe`?

Ideas That Probably Should Be Separate Projects
-----------------------------------------------

* Suggest CI and other stuff via Github Actions, for projects missing such.
* Look for unmaintained crate dependencies, and suggest maintained crates to replace them.
    * Automatically update deps, run unit tests to see if it works.
    * Pull in API tests from old crate to the replacement if missing, to ensure interface contract?
    * Use ML to examine trends in other codebases.  If others have replaced crate `A` with crate `B`, suggest the same.
