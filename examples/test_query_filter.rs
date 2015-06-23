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

use gen::bazaar::Product; 
use rustorm::em::EntityManager;
use rustorm::table::IsTable;
use rustorm::dao::IsDao;
use rustorm::query::Query;
use rustorm::dao::Type;
use rustorm::filter::{Filter,Equality,Operand};

mod gen;


fn main(){
    let pg:Result<Postgres,&str> = Postgres::new("postgres://postgres:p0stgr3s@localhost/bazaar_v6");
       match pg{
        Ok(pg) => {
            let em = EntityManager::new(&pg);
            let mut query = Query::new();
            query.from_table(&Product::table());
            query.enumerate_columns(&Product::table());
            let val = Type::String("iphone%".to_string());
            query.filter(Filter::new("name", Equality::LIKE, Operand::Value(val)));
            let daos = em.retrieve(&mut query);
            for d in daos{
                let prod = Product::from_dao(&d);
                let pid:Uuid = d.get("product_id");
                let name:String = d.get("name");
                println!("\n{} {}", pid, name);
                println!("{}-{}", prod.product_id, prod.name.unwrap());
            }
        }
        Err(error) =>{
            println!("{}",error);
        }
    }
}
