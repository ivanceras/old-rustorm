extern crate rustorm;

use rustorm::query::Query;
use rustorm::pool::ManagedPool;

fn main() {
    let url = "mysql://root:r00t@localhost/bazaar_v6";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();

    let dao = Query::select_all()
                   .from_table(&"product")
                   .retrieve(db.as_ref());

    println!("dao: {:#?}", dao);
}
