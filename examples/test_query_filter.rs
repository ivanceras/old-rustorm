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

mod gen;
 

fn main(){
    let pg:Result<Postgres,&str> = Postgres::new("postgres://postgres:p0stgr3s@localhost/bazaar_v6");
       match pg{
        Ok(pg) => {
            let em = EntityManager::new(&pg);
            let mut query = Query::new();
            query.from_table(&Product::table());
            query.enumerate_table_all_columns(&Product::table());
            let val = Type::String("iphone%".to_string());
            query.add_filter(Filter::new("name", Equality::LIKE, Operand::Value(val)));
            let result = em.retrieve(&mut query);
            let products = Product::from_dao_result(&result);
            
            for p in products{
                println!("{}-{}", p.product_id, p.name.unwrap());
            }
        }
        Err(error) =>{
            println!("{}",error);
        }
    }
}
