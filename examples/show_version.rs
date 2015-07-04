extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use uuid::Uuid;
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use rustc_serialize::json;

use rustorm::db::postgres::Postgres;
use rustorm::query::Query;
use rustorm::query::{Filter,Equality};
use rustorm::dao::{Dao,IsDao};
use rustorm::database::Pool;
use rustorm::database::Database;
 

fn main(){
    let pg = Postgres::connect_with_url("postgres://postgres:p0stgr3s@localhost/bazaar_v6").unwrap();
    let version = pg.version();
    println!("version: {}", version);
}
