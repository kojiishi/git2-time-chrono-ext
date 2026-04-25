//! This crate converts [`git2::Time`] to [`chrono::DateTime`].
//!
//! Please see [`Git2TimeChronoExt`] for references and examples.

mod error;
mod git2_time_chrono_ext;

pub use error::Error;
pub use git2_time_chrono_ext::Git2TimeChronoExt;
