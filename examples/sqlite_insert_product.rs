extern crate rustorm;

use rustorm::query::Query;
use rustorm::pool::ManagedPool;

fn main() {
    let url = "sqlite:///file.db";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();

    let _ = Query::insert()
                 .set("name", &"Test Product")
                 .into_table(&"product")
                 .execute(db.as_ref());
}
