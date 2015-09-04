extern crate rustorm;

use rustorm::query::Query;
use rustorm::query::{Filter,Equality};
use rustorm::pool::ManagedPool;
use rustorm::database::Database;

fn main(){
    let url = "sqlite:///file.db";
    let mut pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();
    
    let sql = "PRAGMA foreign_key_list(product_availability);";
     
    let result = db.as_ref().execute_sql_with_return_columns(sql, &vec![], vec!["id", "seq", "table", "from", "to", "on_update", "on_delete", "match"]);
    
    println!("result: {:#?}", result);
}
