//! Rustorm is a simple ORM implemented in rust.
//! 
//!
//! ```rust,no_run
//! extern crate rustorm;
//! extern crate uuid;
//! extern crate chrono;
//! extern crate rustc_serialize;


//! use rustorm::db::postgres::Postgres;
//! use rustorm::codegen;
//! use uuid::Uuid;
//! use chrono::datetime::DateTime;
//! use chrono::offset::utc::UTC;
//! use rustc_serialize::json;
//! 
//! use rustorm::em::EntityManager;
//! use rustorm::table::IsTable;
//! use rustorm::dao::IsDao;
//! use rustorm::query::Query;
//! use rustorm::dao::Type;
//! use rustorm::query::{Filter,Equality,Operand};
//! use gen::bazaar::Product;
//! use gen::bazaar::ProductAvailability;
//! 
//! mod gen;
//!  
//! 
//! fn main(){
//!     let pg:Result<Postgres,&str> = Postgres::new("postgres://postgres:p0stgr3s@localhost/bazaar_v6");
//!        match pg{
//!         Ok(pg) => {
//!             let em = EntityManager::new(&pg);
//!             let mut query = Query::new();
//!             query.from_table(&Product::table());
//!             query.enumerate_column("*");
//!             
//!             query.left_join(&ProductAvailability::table(), 
//!                 "product.product_id", "product_availability.product_id");
//!             query.filter("product.name", Equality::LIKE, &"iphone%");
//!             query.add_filter(Filter::new("product.description", Equality::LIKE, Operand::Value(Type::String("%Iphone%".to_string()))));
//!             query.desc("product.created");
//!             query.asc("product.product_id");
//!             
//!             let result = em.retrieve(&mut query);
//!             let products = Product::from_dao_result(&result);
//!             
//!             for p in products{
//!                 println!("{}-{}", p.product_id, p.name.unwrap());
//!             }
//!         }
//!         Err(error) =>{
//!             println!("{}",error);
//!         }
//!     }
//! }
//! ```
//!
//!
//!

extern crate postgres;
extern crate rustc_serialize;
extern crate uuid;
extern crate chrono;
extern crate regex;
extern crate url;


pub mod em;
pub mod query;
pub mod dao;
pub mod database;
pub mod db;
pub mod table;
pub mod writer;
pub mod codegen;
