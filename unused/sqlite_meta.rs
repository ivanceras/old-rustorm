extern crate rustorm;

use rustorm::pool::ManagedPool;
use rustorm::database::Database;

fn main() {
    let url = "sqlite:///file1.db";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();

    let sql = "PRAGMA table_info(product);";
    //let sql = "SELECT sql FROM sqlite_master WHERE tbl_name = 'table_name' AND type = 'table'";

    let result = db.as_ref().execute_sql_with_return(sql, &vec![]);

    println!("result: {:#?}", result);
}
