extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use rustorm::query::Query;
use rustorm::query::{Filter,Equality};
use rustorm::dao::{Dao,IsDao};
use rustorm::pool::Pool;



fn main(){
    let mut pool = Pool::init();
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    let db = pool.get_db_with_url(&url).unwrap();
    match Query::delete()
        .from_table("bazaar.category")
            .filter("name", Equality::LIKE, &"Test%")
        .execute(db.as_ref()){
            
        Ok(x) => println!("deleted {}", x),
        Err(e) => println!("Error {}", e)
    };
    pool.release(db);
}
