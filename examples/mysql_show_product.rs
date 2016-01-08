extern crate rustorm;

use rustorm::query::Query;
use rustorm::pool::ManagedPool;

// run using cargo run --release --example mysql_show_product --features mysql
fn main() {
    let url = "mysql://root:r00t@localhost/bazaar_v8";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();

    let dao = Query::select_all()
                   .from_table(&"product")
                   .retrieve(db.as_ref());

    println!("dao: {:#?}", dao);
}
