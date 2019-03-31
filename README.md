# auto-clippy

Automatically find and fix problems in the Rust code ecosystem.

# Planned Features

* Clone software repos, and pull updates when available.
* Scan source code using Rust's clippy and other tools.
* Run spelling and grammer checks on documentation, for human review.
* Create fixes for problems found when possible.
    * Create new appropriately-named branch.
    * Group related fixes together in a single commit and push.
    * (Disabled by default) Automatically create pull requests (github).
* Track work in a database (defaults to local sqlite3)
    * When re-run, check which repos need to have changes pulled, and tests re-run.
    * For example: If the clippy version hasn't changed, and the software repo hasn't been updated, it isn't necessary to run clippy again on this repo.
    * Track pull request status (github).
* Support all source code sharing and CI sites (github, gitlab, bitbucket, etc.).
* Allow multiple instances to be run in parallel.
    * All work status updates managed through the database.
    * Support distributed databases for even greater scaling.
