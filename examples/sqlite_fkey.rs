extern crate rustorm;

use rustorm::pool::ManagedPool;
use rustorm::database::Database;

fn main() {
    let url = "sqlite:///file.db";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();

    let sql = "PRAGMA foreign_key_list(product_availability);";

    let result = db.as_ref().execute_sql_with_return(sql, &vec![]);

    println!("result: {:#?}", result);
}
