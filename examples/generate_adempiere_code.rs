extern crate rustorm;
extern crate uuid;

use rustorm::db::postgres::Postgres;
use rustorm::database::DatabaseDev;
use rustorm::table;
use uuid::Uuid;
use std::fs::File;
use std::io::Write;
use rustorm::codegen;

fn main(){
    let pg = Postgres::new("postgres://postgres:p0stgr3s@localhost/adempiere");
    match pg{
        Ok(pg) => {
             codegen::generate_all(&pg,"./examples/adempiere.rs", "./examples/adempiere_is_table.rs");
        },
        Err(err) => {
            println!("{}",err);
        },
    };
}

