extern crate rustorm;

use rustorm::platform::postgres::Postgres;
use rustorm::query::Query;
use rustorm::query::{Filter,Equality};

fn main(){
    let pg = Postgres::connect_with_url("postgres://postgres:p0stgr3s@localhost/bazaar_v6").unwrap();
        
        Query::insert()
            .set("name", &"Test Category")
        .into_table(&"bazaar.category")
            .execute(&pg);
}
