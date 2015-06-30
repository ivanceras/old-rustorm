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
use gen::bazaar::Category;
use gen::bazaar::category;
use rustorm::dao::Dao;

mod gen;
 

fn main(){
    let pg:Result<Postgres,&str> = Postgres::new("postgres://postgres:p0stgr3s@localhost/bazaar_v6");
       match pg{
        Ok(pg) => {
            let em = EntityManager::new(&pg);
            let mut dao = Dao::new();
            dao.set("name", &"test category");
            em.insert(&Category::table(), dao);
            
            let mut query = Query::delete();
            query.from::<Category>();
            query.filter(category::name, Equality::LIKE, &"test%");
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
