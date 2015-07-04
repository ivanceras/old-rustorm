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

mod gen;
 

fn main(){
    let pg= Postgres::connect_with_url("postgres://postgres:p0stgr3s@localhost/bazaar_v6").unwrap();
    
    // we can not cast right away to product since other columns are ommited, therefore can not create 
    // a product instance
    let dao = Query::select()
        .enumerate(vec![product::name, product::description, product::product_id])
        .from(&Product::table())
        .filter(product::name, Equality::EQ, &"GTX660 Ti videocard")
        .execute_with_one_return(&pg);
        
    println!("debug: {:?}", dao);
    let product_id: Uuid = dao.get("product_id");
    let name: String = dao.get("name");
    let description: String = dao.get("description");
    println!("{}  {}  {}", product_id, name, description );
}
