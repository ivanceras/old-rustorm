extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use uuid::Uuid;
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use rustc_serialize::json;

use rustorm::query::Query;
use rustorm::query::{Filter,Equality};
use rustorm::dao::{Dao,IsDao};
use rustorm::database::Pool;

#[derive(Debug, Clone)]
pub struct Photo {
    pub photo_id:Uuid,
    pub url:Option<String>,
}

impl IsDao for Photo{
    fn from_dao(dao:&Dao)->Self{
        Photo{
            photo_id: dao.get("photo_id"),
            url: dao.get_opt("url"),
        }
    }
}

fn main(){
    let mut pool = Pool::init();
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    let db = pool.get_db_with_url(&url).unwrap();
    
    let photo: Photo = Query::select_all()
                        .enumerate_column("photo.url")
                        .from_table("bazaar.product")
                        .left_join("bazaar.product_photo",
                            "product.product_id", "product_photo.product_id")
                        .left_join("bazaar.photo",
                            "product_photo.photo_id", "photo.photo_id")
                        .filter("product.name", Equality::EQ, &"GTX660 Ti videocard")
                        .collect_one(db.as_ref());
                        
    println!("photo: {} {}",photo.photo_id, photo.url.unwrap());
    pool.release(db);
}