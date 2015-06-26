extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;
extern crate postgres;


use rustorm::db::postgres::Postgres;
use rustorm::codegen;
use uuid::Uuid;
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use rustc_serialize::json;

use rustorm::em::EntityManager;
use rustorm::table::IsTable;
use rustorm::dao::IsDao;
use rustorm::dao::Dao;
use rustorm::query::Query;
use rustorm::dao::Type;
use rustorm::query::{Filter,Equality,Operand};
use gen::bazaar::Product;
use gen::bazaar::ProductAvailability;
use postgres::types::ToSql;

mod gen;
 

fn main(){
    let pg:Result<Postgres,&str> = Postgres::new("postgres://postgres:p0stgr3s@localhost/bazaar_v6");
       match pg{
        Ok(pg) => {
            let sql = "INSERT INTO bazaar.product ( product_id, name) VALUES( $1 , $2 )";
            let pid = Uuid::new_v4();
            let name = "test product uuid";
            let params:Vec<&ToSql> = vec![&pid, &name];
            let result = pg.conn.execute(&sql, &params);
            match result{
                Ok(x) => println!("ok {}",x),
                Err(e) => println!("error: {:?}", e),
            };
        }
        Err(error) =>{
            println!("{}",error);
        }
    }
}
