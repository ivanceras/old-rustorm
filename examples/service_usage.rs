extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use uuid::Uuid;

use rustorm::query::Query;
use rustorm::query::{Filter,Equality,Operand};
use rustorm::pool::Pool;
use rustorm::database::Database;
use rustorm::dao::{IsDao, Dao};

use std::sync::{Arc, Mutex};
use std::thread;


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
fn main(){
    let mut pool = Arc::new(Mutex::new(Pool::init()));
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    
    let pool1 = pool.clone();
    
    let db = pool1.lock().unwrap().from_url(&url);
    
    match db{
            Ok(db) => {
                thread::spawn(move || {
                    show_product(db.as_ref());//borrow a database
                    pool1.lock().unwrap().release(db);//borrow has ended, release it
                });
            
         }
        Err(e) => {
            println!("Unable to connect to database {}", e);
        }
    };
    
    let pool2 = pool.clone();
    let db = pool2.lock().unwrap().from_url(&url);
    match db{
            Ok(db) => {
                thread::spawn(move || {
                    show_all_product(db.as_ref());//borrow a database
                    pool2.lock().unwrap().release(db);//borrow has ended, release it
                });
            
         }
        Err(e) => {
            println!("Unable to connect to database {}", e);
        }
    };
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

/// a dispatched controller with an accesss to a database reference
fn show_all_product(db: &Database){
    let prod: Product = Query::select_all()
        .from_table("bazaar.product")
        .collect_one(db);

    println!("{}  {}  {:?}", prod.product_id, prod.name.unwrap(), prod.description);
    
}