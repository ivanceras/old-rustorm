extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use rustorm::pool::ManagedPool;
use rustorm::database::Database;


fn main() {
    let url = "mysql://test:test@localhost/test";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();
    let version = db.as_ref().version().unwrap();
    println!("version: {}", version);
}
