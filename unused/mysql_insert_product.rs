extern crate rustorm;

use rustorm::query::Query;
use rustorm::pool::ManagedPool;

fn main() {
    let url = "mysql://root:r00t@localhost/bazaar_v8";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();

    let _ = Query::INSERT()
                 .SET("name", &"Test Product")
                 .SET("created_by", &10000)
                 .INTO(&"product")
                 .execute(db.as_ref());
}
