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
    let pg:Result<Postgres,&str> = Postgres::new("postgres://postgres:p0stgr3s@localhost/bazaar_v6");
       match pg{
        Ok(pg) => {
            let em = EntityManager::new(&pg);
            let mut query = Query::update();
            query.from::<Product>();
            query.enumerate_column(product::name);
            query.enumerate_all_table_column_as_return(&Product::table());
            query.value(&"iphone");
            query.filter(product::name, Equality::LIKE, &"aphone");
            
            query.add_filter(
                Filter::new(product::description, Equality::LIKE, 
                    Operand::Value(Type::String("%Iphone%".to_string())))
                );
            let result:Vec<Product> = query.collect(&pg);
            println!("{:?}",result);
        }
        Err(error) =>{
            println!("{}",error);
        }
    }
}
