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
use gen::bazaar::Product;
use gen::bazaar::ProductAvailability;
use gen::bazaar::product;
use gen::bazaar::product_availability;
use gen::bazaar::ProductPhoto;
use gen::bazaar::product_photo;
use gen::bazaar::Photo;
use gen::bazaar::photo;

mod gen;
 

fn main(){
    let pg= Postgres::connect_with_url("postgres://postgres:p0stgr3s@localhost/bazaar_v6").unwrap();
     let photo: Photo = Query::select()
                        .from(&Product::table())
                        .left_join(&ProductPhoto::table(),
                            product::product_id, product_photo::product_id)
                        .left_join(&Photo::table(), 
                            product_photo::photo_id, photo::photo_id)
                        .filter(product::name, Equality::EQ, &"GTX660 Ti videocard")
                        .collect_one(&pg);
    println!("debug: {:?}", photo);
}
