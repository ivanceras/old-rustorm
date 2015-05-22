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
	let pg:Result<Postgres,&str> = Postgres::new("postgres://postgres:p0stgr3s@localhost/bazaar_v5");
	match pg{
		Ok(pg) => {
			codegen::generate_all_tables(pg,"./examples/gen.rs");
		}
		Err(error) =>{
			println!("{}",error);
		}
	}
}

