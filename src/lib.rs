//! Rustorm is a simple ORM implemented in rust.
//!
//!
//!
//!

extern crate rustc_serialize;
extern crate postgres;
#[cfg(feature = "sqlite")]
extern crate rusqlite;
#[cfg(feature = "mysql")]
extern crate mysql;
extern crate uuid;
extern crate chrono;
extern crate regex;
extern crate url;
extern crate r2d2;
extern crate r2d2_postgres;
#[cfg(feature = "sqlite")]
extern crate r2d2_sqlite;
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
pub mod pool;
