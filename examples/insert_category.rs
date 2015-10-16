extern crate rustorm;

use rustorm::query::Query;
use rustorm::query::{Filter, Equality};
use rustorm::pool::ManagedPool;

fn main() {
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    let mut pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();

    Query::insert()
        .set("name", &"Test Category112")
        .into_table(&"bazaar.category")
        .execute(db.as_ref());
}
