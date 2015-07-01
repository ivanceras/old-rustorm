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
    let pg = Postgres::with_connection("postgres://postgres:p0stgr3s@localhost/bazaar_v6");
    let sql = "INSERT INTO bazaar.product ( name, description ) VALUES( $1 , $2 ) RETURNING *";//product_id, now(), created, updated";
    let pid = Uuid::new_v4();
    let name = "product 1234";
    let description = "more info of the test product";
    let params:Vec<&ToSql> = vec![&name, &description];
    let stmt = pg.conn.prepare(&sql).unwrap();
    for row in stmt.query(&params).unwrap() {
        let mut index = 0;
        for column in row.columns(){
            println!("{:?}", column);
            index += 1;
        }
    }
}
