extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;


use rustorm::db::postgres::Postgres;
use rustorm::codegen;
use uuid::Uuid;
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use rustc_serialize::json;

use rustorm::em::EntityManager;
use rustorm::table::IsTable;
use rustorm::dao::IsDao;
use rustorm::query::Query;
use rustorm::dao::Type;
use rustorm::query::{Filter,Equality,Operand};
use gen::bazaar::Product;
use gen::bazaar::ProductAvailability;
use gen::bazaar::product;
use gen::bazaar::product_availability;

use rustorm::database::Pool;
use rustorm::database::Database;

mod gen;

/// on a webserver this will be the main thread, where it instantiate
/// the connection pool in the entirety of the application
/// when a request in made, a thread is spawned for that request
/// with an access to the a connection pool 
fn main(){
    let mut pool = Pool::init();
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    let db = pool.get_db_with_url(&url);
    match db{
            Ok(db) => {
            show_product(db.as_ref());//borrow a database
            println!("There are {} free connection", pool.total_free_connections());
            pool.release(db);//borrow has ended, release it
            println!("There are {} free connection", pool.total_free_connections());
         }
        Err(e) => {
            println!("Unable to connect to database {}", e);
        }
    }
}

/// a dispatched controller with an accesss to a database reference
fn show_product(db: &Database){
    let prod: Product = Query::select_all()
        .from_table("bazaar.product")
        .filter("name", Equality::EQ, &"GTX660 Ti videocard")
        .collect_one(db);

    println!("{}  {}  {:?}", prod.product_id, prod.name.unwrap(), prod.description);
    
}