//! Rustorm is a simple ORM implemented in rust.
//!
//!
//!
//!

extern crate rustc_serialize;
#[cfg(feature = "postgres")]
extern crate postgres;
#[cfg(feature = "sqlite")]
extern crate rusqlite;
#[cfg(feature = "mysql")]
extern crate mysql;
extern crate uuid;
extern crate chrono;
extern crate regex;
extern crate url;
extern crate time;
#[macro_use]
extern crate log;



// pub mod em;
pub mod query;
pub mod dao;
pub mod database;
pub mod platform;
pub mod table;
pub mod writer;
pub mod config;
