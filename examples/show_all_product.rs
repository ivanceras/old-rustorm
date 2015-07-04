extern crate rustorm;
extern crate uuid;
extern crate chrono;
// extern crate rustc_serialize;

use uuid::Uuid;
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
// use rustc_serialize::json;

// use rustorm::platform::postgres::Postgres;
use rustorm::query::Query;
// use rustorm::query::{Filter,Equality};
use rustorm::dao::{Dao,IsDao};
use rustorm::database::Pool;



#[derive(Debug, Clone)]
pub struct Product {
    pub product_id:Uuid,
    pub barcode:Option<String>,
    pub currency_id:Option<Uuid>,
    pub info:Option<String>,
    pub is_service:Option<bool>,
    pub owner_id:Option<Uuid>,
    pub parent_product_id:Option<Uuid>,
    pub price:Option<f64>,
    pub seq_no:Option<i32>,
    pub tags:Option<String>,
    pub unit:Option<String>,
    pub upfront_fee:Option<f64>,
    pub use_parent_price:Option<bool>,
    pub active:bool,
    pub client_id:Option<Uuid>,
    pub created:DateTime<UTC>,
    pub created_by:Option<Uuid>,
    pub description:Option<String>,
    pub help:Option<String>,
    pub name:Option<String>,
    pub organization_id:Option<Uuid>,
    pub priority:Option<f64>,
    pub updated:DateTime<UTC>,
    pub updated_by:Option<Uuid>,
}

impl IsDao for Product{
    fn from_dao(dao:&Dao)->Self{
        Product{
            organization_id: dao.get_opt("organization_id"),
            client_id: dao.get_opt("client_id"),
            created: dao.get("created"),
            created_by: dao.get_opt("created_by"),
            updated: dao.get("updated"),
            updated_by: dao.get_opt("updated_by"),
            priority: dao.get_opt("priority"),
            name: dao.get_opt("name"),
            description: dao.get_opt("description"),
            help: dao.get_opt("help"),
            active: dao.get("active"),
            product_id: dao.get("product_id"),
            parent_product_id: dao.get_opt("parent_product_id"),
            is_service: dao.get_opt("is_service"),
            price: dao.get_opt("price"),
            use_parent_price: dao.get_opt("use_parent_price"),
            unit: dao.get_opt("unit"),
            tags: dao.get_opt("tags"),
            info: dao.get_opt("info"),
            seq_no: dao.get_opt("seq_no"),
            upfront_fee: dao.get_opt("upfront_fee"),
            barcode: dao.get_opt("barcode"),
            owner_id: dao.get_opt("owner_id"),
            currency_id: dao.get_opt("currency_id"),
        }
    }
}

fn main(){
    let mut pool = Pool::init();
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    let db = pool.get_db_with_url(&url).unwrap();
    
    let products: Vec<Product> = Query::select_all()
            .from_table("bazaar.product")
            .collect(db.as_ref());
    
    for prod in products{
        let name = prod.name.unwrap();
        let desc = match prod.description{
                        Some(desc) => desc,
                        None => "".to_string()
                    };
        println!("{}  {}  {:?}", prod.product_id, name, desc);
    }
    pool.release(db);
}