extern crate rustorm;

use rustorm::db::postgres::Postgres;
use rustorm::query::Query;
use rustorm::query::{Filter,Equality};

fn main(){
    let pg = Postgres::connect_with_url("postgres://postgres:p0stgr3s@localhost/bazaar_v6").unwrap();
    match Query::delete()
        .from_table("bazaar.category")
            .filter("name", Equality::LIKE, &"Test%")
        .execute(&pg){
            
        Ok(x) => println!("deleted {}", x),
        Err(e) => println!("Error {}", e)
    
    }
}
