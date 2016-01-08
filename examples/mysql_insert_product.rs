extern crate rustorm;

use rustorm::query::Query;
use rustorm::pool::ManagedPool;

fn main() {
    let url = "mysql://root:r00t@localhost/bazaar_v8";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();

    let _ = Query::insert()
                 .set("name", &"Test Product")
                 .set("created_by", &10000)
                 .into_table(&"product")
                 .execute(db.as_ref());
}
