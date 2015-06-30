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
            let mut query = Query::delete();
            query.from::<Product>();
            query.filter(product::name, Equality::LIKE, &"iphone");
            let sql = query.build(&pg);
            println!("SQL FRAG: {}", sql);
            match query.execute(&pg){
                Ok(x) => println!("{:?}",x),
                Err(e) => println!("Error {:?}",e),
            };
        }
        Err(error) =>{
            println!("{}",error);
        }
    }
}
