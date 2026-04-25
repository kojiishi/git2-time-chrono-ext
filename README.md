[![CI-badge]][CI]
[![crate-badge]][crate]
[![docs-badge]][docs]

[CI-badge]: https://github.com/kojiishi/git2-time-chrono-ext/actions/workflows/rust-ci.yml/badge.svg
[CI]: https://github.com/kojiishi/git2-time-chrono-ext/actions/workflows/rust-ci.yml
[crate-badge]: https://img.shields.io/crates/v/git2-time-chrono-ext.svg
[crate]: https://crates.io/crates/git2-time-chrono-ext
[docs-badge]: https://docs.rs/git2-time-chrono-ext/badge.svg
[docs]: https://docs.rs/git2-time-chrono-ext/

# git2-time-chrono-ext

Rust extension library to convert [`git2::Time`] to [`chrono`].

[`chrono`]: https://docs.rs/chrono/latest/chrono/
[`git2::Time`]: https://docs.rs/git2/latest/git2/struct.Time.html

## Install

```shell-session
cargo add git2-time-chrono-ext
```

Please see the [releases] for the change history.

[releases]: https://github.com/kojiishi/git2-time-chrono-ext/releases

## Example

## Usage

```rust
use git2_time_chrono_ext::Git2TimeChronoExt;

// Print `git2::Time` to `stdout`.
fn print_git2_time(time: git2::Time) {
  println!("{}", time.to_local_date_time().unwrap());
}

// Convert `git2::Time` to `Stirng` in the specified format.
fn git2_time_to_string(time: git2::Time) -> String {
  time.to_local_date_time().unwrap().format("%Y-%m-%d %H:%M").to_string()
}
```
Please see the [docs] for more details.
