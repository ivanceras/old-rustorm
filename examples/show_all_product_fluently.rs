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
    let products:Vec<Product> = Query::select()
                .enumerate_table_all_columns(&Product::table())
                .from(&Product::table())
                .collect(&pg);
                
    for prod in products{
        println!("{}  {}  {:?}", prod.product_id, prod.name.unwrap(), prod.description);
    }
}
