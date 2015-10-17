extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use uuid::Uuid;
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use rustc_serialize::json;

use rustorm::query::Query;
use rustorm::query::{Filter, Equality};
use rustorm::dao::{Dao, IsDao};
use rustorm::pool::ManagedPool;
use rustorm::database::Database;


fn main() {
    let url = "mysql://test:test@localhost/test";
    let mut pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();
    let version = db.as_ref().version();
    println!("version: {}", version);
}
