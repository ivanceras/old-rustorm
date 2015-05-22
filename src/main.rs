extern crate postgres;
extern crate rustc_serialize;
extern crate uuid;
extern crate chrono;
extern crate regex;

use types::Dao;
use types::Types;

mod em;
mod filter;
mod query;
mod table;
mod types;
mod database;
mod meta;
mod join;
mod db;
mod writer;
mod gen;
mod codegen;


fn main(){
	println!("RustORM is an simple ORM for rust");
	let title = "engr";
	let mut dao = Dao::new();
	dao.set_value("title", &title);

	dao.values.insert("first_name", Types::String("lee".to_string()));
	dao.values.insert("last_name", Types::String("cesar".to_string()));
	dao.values.insert("age", Types::U32(29));
	
	println!("first_name: {:?}", dao.get_value("first_name").unwrap());
	println!("last_name: {:?}", dao.get_value("last_name"));
	println!("title: {:?}", dao.get_value("title"));
	println!("age: {:?}", dao.get_value("age"));
	
	
}
