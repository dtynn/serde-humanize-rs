//! A Serde deserializer for duration and byte-size using the crate [`humanize-rs`].
//!
//! # Example
//! ```
//! extern crate serde;
//! #[macro_use] extern crate serde_derive;
//! extern crate serde_json;
//! extern crate serde_humanize_rs;
//!
//! use std::time::{Duration, SystemTime};
//!
//! #[derive(Deserialize)]
//! struct Config {
//!     #[serde(with = "serde_humanize_rs")]
//!     size: usize,
//!
//!     #[serde(with = "serde_humanize_rs")]
//!     interval: Duration,
//!
//!     #[serde(with = "serde_humanize_rs")]
//!     close_at: SystemTime,
//! }
//!
//! let json = r#"{"size": "1 M", "interval": "1h30m", "close_at": "2105-03-01T10:23:57.000013579+08:00"}"#;
//! let cfg = serde_json::from_str::<Config>(json).unwrap();
//! assert_eq!(cfg.size, 1_000_000);
//! assert_eq!(cfg.interval, Duration::from_secs(5400));
//! assert_eq!(
//!     cfg.close_at.duration_since(SystemTime::UNIX_EPOCH).unwrap(),
//!     Duration::new(4265317437, 000013579)
//! );
//! ```
//!
//! [`humanize-rs`]: https://github.com/dtynn/humanize-rs

#![warn(missing_docs)]

extern crate humanize_rs;
extern crate serde;

#[cfg(test)]
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
extern crate serde_json;

mod bytes;
mod de;
mod duration;
mod time;

#[cfg(test)]
mod tests;

pub use de::deserialize;
