extern crate rustorm;

use rustorm::query::Query;
use rustorm::pool::ManagedPool;

fn main() {
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v8";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();

    let _ = Query::INSERT()
                 .SET("name", &"Test Category112")
                 .INTO_TABLE(&"bazaar.category")
                 .execute(db.as_ref());
}
