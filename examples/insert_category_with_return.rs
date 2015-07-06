extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use uuid::Uuid;
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;

use rustorm::query::Query;
use rustorm::dao::{Dao,IsDao};
use rustorm::pool::Pool;

#[derive(Debug, Clone)]
struct Category {
    pub category_id:Uuid,
    pub name:Option<String>,
    pub active:bool,
    pub client_id:Option<Uuid>,
    pub created:DateTime<UTC>,
    pub created_by:Option<Uuid>,
    pub description:Option<String>,
    pub help:Option<String>,
    pub organization_id:Option<Uuid>,
    pub priority:Option<f64>,
    pub updated:DateTime<UTC>,
    pub updated_by:Option<Uuid>,
}

impl IsDao for Category{
    fn from_dao(dao:&Dao)->Self{
        Category{
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
            category_id: dao.get("category_id"),
        }
    }
}


fn main(){
    let mut pool = Pool::init();
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    let db = pool.from_url(&url).unwrap();
        
    let category: Category = Query::insert()
            .set("name", &"Test Category11")
        .into_table(&"bazaar.category")
            .return_all()
            .collect_one(db.as_ref());
    println!("category: {}", category.name.unwrap());
}
