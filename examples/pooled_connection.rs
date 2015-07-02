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

mod gen;

fn main(){
    let mut pool = Pool::init();
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    println!("{} connections..", pool.total_pool_connection());
    let db = pool.get_db_with_url(&url);
    
        let mut query = Query::select();
    query.select_all()
        .from::<Product>()
        .filter(product::name, Equality::EQ, &"GTX660 Ti videocard");
    
    let db = db.unwrap();
    let prod: Product =  query.collect_one(db.db.as_ref());
    println!("{}  {}  {:?}", prod.product_id, prod.name.unwrap(), prod.description);
    
    pool.release(db);
}