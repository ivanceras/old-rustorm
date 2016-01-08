extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use rustorm::query::Query;
use rustorm::query::Equality;
use rustorm::pool::ManagedPool;



fn main() {
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v8";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();
    match Query::delete()
               .from_table("bazaar.category")
               .filter("name", Equality::LIKE, &"Test%")
               .execute(db.as_ref()) {

        Ok(x) => println!("deleted {}", x),
        Err(e) => println!("Error {}", e),
    }
}
