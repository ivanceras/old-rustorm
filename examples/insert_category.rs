extern crate rustorm;

use rustorm::query::Query;
use rustorm::query::{Filter,Equality};
use rustorm::database::Pool;

fn main(){
    let mut pool = Pool::init();
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    let db = pool.get_db_with_url(&url).unwrap();
        
        Query::insert()
            .set("name", &"Test Category")
        .into_table(&"bazaar.category")
            .execute(db.as_ref());
}
