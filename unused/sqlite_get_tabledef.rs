extern crate rustorm;

use rustorm::pool::ManagedPool;
#[allow(unused_imports)]
use rustorm::database::Database;

// run using cargo run --release --features sqlite --example sqlite_get_tabledef
fn main() {
    let url = "sqlite:///file.db";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();

    //let table = db.as_dev().get_table_metadata("","product_availability", false);
    let table = db.as_dev().get_table_metadata("", "product", false);

    println!("table: {:#?}", table);
}
