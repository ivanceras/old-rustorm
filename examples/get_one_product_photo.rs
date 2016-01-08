extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use uuid::Uuid;

use rustorm::query::Query;
use rustorm::query::Equality;
use rustorm::dao::{Dao, IsDao};
use rustorm::pool::ManagedPool;
use rustorm::table::{IsTable, Table};
use rustorm::dao::Value;
use rustorm::query::Operand;

#[derive(Debug, Clone)]
pub struct Photo {
    pub photo_id: Uuid,
    pub url: Option<String>,
}

impl IsDao for Photo{
    fn from_dao(dao: &Dao) -> Self {
        Photo {
            photo_id: dao.get("photo_id"),
            url: dao.get_opt("url"),
        }
    }

    fn to_dao(&self) -> Dao {
        let mut dao = Dao::new();
        dao.set("photo_id", &self.photo_id);
        match self.url {
            Some(ref _value) => dao.set("url", _value),
            None => dao.set_null("url"),
        }
        dao
    }
}

impl IsTable for Photo{
    fn table() -> Table {
        Table {
            schema: Some("bazaar".to_string()),
            name: "photo".to_string(),
            parent_table: None,
            sub_table: vec![],
            comment: None,
            columns: vec![],
            is_view: false,
        }
    }
}

fn main() {
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v8";
    let pool = ManagedPool::init(url, 1).unwrap();
    let db = pool.connect().unwrap();

    let photo: Photo = Query::select_all()
                           .column("photo.url")
                           .from_table("bazaar.product")
                           .left_join_table("bazaar.product_photo",
                                            "product.product_id",
                                            "product_photo.product_id")
                           .left_join_table("bazaar.photo",
                                            "product_photo.photo_id",
                                            "photo.photo_id")
                           .filter("product.name", Equality::EQ, &"GTX660 Ti videocard")
                           .collect_one(db.as_ref())
                           .unwrap();

    println!("photo: {} {}", photo.photo_id, photo.url.unwrap());
}
