extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use rustorm::query::Query;
use rustorm::query::{Filter,Equality};
use rustorm::dao::{Dao,IsDao};
use rustorm::pool::ManagedPool;



fn main(){
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    let mut pool = ManagedPool::init(&url, 1);
    let mut db = pool.connect().unwrap();
    match Query::delete()
        .from_table("bazaar.category")
            .filter("name", Equality::LIKE, &"Test%")
        .execute(db.as_mut()){
            
        Ok(x) => println!("deleted {}", x),
        Err(e) => println!("Error {}", e)
    };
}
