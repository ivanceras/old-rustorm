extern crate rustorm;

use rustorm::query::Query;
use rustorm::query::{Filter,Equality};
use rustorm::pool::ManagedPool;
use rustorm::database::Database;

fn main(){
    let url = "sqlite:///file1.db";
    let mut pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();
    
    let table = db.as_dev().get_table_metadata("","product", false);
    
    println!("table: {:#?}", table);
}
