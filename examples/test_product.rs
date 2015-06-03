extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use rustorm::db::postgres::Postgres;
use rustorm::codegen;
use uuid::Uuid;
use rustorm::gen::structs::Product;
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use rustc_serialize::json;


fn main(){
    let pg:Result<Postgres,&str> = Postgres::new("postgres://postgres:p0stgr3s@localhost/bazaar_v6");
    match pg{
        Ok(pg) => {
            let sql = "select * from bazaar.product".to_string();
            let stmt = pg.conn.prepare(&sql).unwrap();
            for row in stmt.query(&[]).unwrap() {
                let product = Product{
                    product_id: row.get("product_id"),
                    barcode: row.get_opt("barcode").ok(),
                    currency_id: row.get_opt("currency_id").ok(),
                    info: row.get_opt("info").ok(),
                    is_service: row.get_opt("is_service").ok(),
                    owner_id: row.get_opt("owner_id").ok(),
                    parent_product_id: row.get_opt("parent_product_id").ok(),
                    price: row.get_opt("price").ok(),
                    seq_no: row.get_opt("seq_no").ok(),
                    tags: row.get_opt("tags").ok(),
                    unit: row.get_opt("unit").ok(),
                    upfront_fee: row.get_opt("upfront_fee").ok(),
                    use_parent_price: row.get_opt("use_parent_price").ok(),
                    active: row.get("active"),
                    client_id: row.get_opt("client_id").ok(),
                    created: row.get("created"),
                    createdby: row.get_opt("createdby").ok(),
                    description: row.get_opt("description").ok(),
                    help: row.get_opt("help").ok(),
                    name: row.get_opt("name").ok(),
                    organization_id: row.get_opt("organization_id").ok(),
                    priority:row.get_opt("priority").ok(),
                    updated:row.get("updated"),
                    updatedby: row.get_opt("updatedby").ok(),
                    owner:None,
                    currency:None,
                    product_availability:None,
                    category:None,
                    photo:None,
                    review:None
                };
                
                println!("{},", json::as_pretty_json(&product));
            }
        }
        Err(error) =>{
            println!("{}",error);
        }
    }
}

