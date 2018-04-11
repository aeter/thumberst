# thumberst

A program modelled after [thumper](https://github.com/migrateup/thumper).
I wanted to check out how easy it would be to migrate most of it to Rust.
Also wanted to do a speed comparison between the tools. :rocket:

## Installation and running
```
[~] rustc --version
rustc 1.25.0
```

```
git clone <repo_url>
mkdir <dest_dir>
cargo build --release
cargo run --release -- --src_dir <src_dir> --dest_dir <dest_dir>
```
