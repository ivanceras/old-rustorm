extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use uuid::Uuid;
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use rustc_serialize::json;

use rustorm::query::Query;
use rustorm::query::{Filter,Equality};
use rustorm::dao::{Dao,IsDao};
use rustorm::pool::ManagedPool;
use rustorm::database::Database;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;
use rustorm::pool::Platform;



#[derive(Debug, Clone)]
pub struct Product {
    pub product_id:Uuid,
    pub name:Option<String>,
    pub description:Option<String>,
}

impl IsDao for Product{
    fn from_dao(dao:&Dao)->Self{
        Product{
            product_id: dao.get("product_id"),
            name: dao.get_opt("name"),
            description: dao.get_opt("description"),
        }
    }
}


/// on a webserver this will be the main thread, where it instantiate
/// the connection pool in the entirety of the application
/// when a request in made, a thread is spawned for that request
/// with an access to the a connection pool 

///TODO: need to revisit rust concurrency
fn main(){
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    let mut pool = Arc::new(Mutex::new(ManagedPool::init(url, 5)));
    for i in 0..3000{
    	let pool = pool.clone();
        let db: Platform = pool.lock().unwrap().connect().unwrap();//important to obtain a connection before opening a thread
        thread::spawn(move || {
                println!("spawning thread {}", i);
                show_product(db.as_ref());//borrow a database
        });
    }
     thread::sleep_ms(5000);
}


/// a dispatched controller with an accesss to a database reference
fn show_product(db: &Database){
    let prod: Product = Query::select_all()
        .from_table("bazaar.product")
        .filter("name", Equality::EQ, &"GTX660 Ti videocard")
        .collect_one(db);

    println!("{}  {}  {:?}", prod.product_id, prod.name.unwrap(), prod.description);
 }