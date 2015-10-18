extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use rustorm::pool::ManagedPool;
use rustorm::database::Database;


fn main() {
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();
    let version = db.as_ref().version();
    println!("version: {}", version);
}
