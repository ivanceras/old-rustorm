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
use rustorm::database::Database;

mod gen;
 

fn main(){
    let pg = Postgres::connect_with_url("postgres://postgres:p0stgr3s@localhost/bazaar_v6").unwrap();
    let sql = "SHOW server_version";
    let dao = pg.execute_sql_with_one_return(sql, &vec![]);
    println!("{:?}",dao);
}
