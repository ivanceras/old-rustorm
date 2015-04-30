extern crate postgres;

use dao::DAO;
use postgres::ToSql;
use postgres::types::Type;
use std::fs::File;
use std::io::Write;


mod dao;
mod db;
mod database;
mod filter;
mod query;
mod table;
mod meta;
mod join;



fn main(){
	println!("RustORM is an simple ORM for rust");
	
	let dao = DAO::new();
	//let mut file:Write = File::create("foo.txt").unwrap();
	//let v = "cesar".to_string().to_sql(&Type::Text, file);
	//dao.set("lee".to_string(), v);
}
