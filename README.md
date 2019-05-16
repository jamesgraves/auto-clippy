# auto-clippy

Automatically find and fix problems in the Rust code ecosystem.

* [Design Overview][design]

[design]: docs/Design_Overview.md

# Prerequisites

*aspell and sqlite3*

Debian / Ubuntu:

```
sudo apt install aspell libsqlite3-dev
```

*[Diesel ORM](http://diesel.rs/)*

```
cargo install diesel_cli --no-default-features --features sqlite
diesel setup
diesel migration run
```
