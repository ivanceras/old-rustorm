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
use rustorm::pool::ManagedPool;

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
    fn to_dao(&self)->Dao{
        let mut dao = Dao::new();
        dao.set("photo_id", &self.photo_id);
        match self.url{
            Some(ref _value) => dao.set("url", _value),
            None => dao.set_null("url"),
        };
        dao
    }
}

fn main(){
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    let mut pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();
    
    let mut query = Query::insert();
    
    query.into_table("bazaar.product")
        .set("name", &"product1")
        .returns(vec!["category.name"]);
        
    let frag = query.build(db.as_ref());
    
    let expected = "
   INSERT INTO bazaar.product( name ) 
   VALUES ($1 ) 
RETURNING name
".to_string();
    println!("actual:   {{\n{}}} [{}]", frag.sql, frag.sql.len());
    println!("expected: {{{}}} [{}]", expected, expected.len());
    assert!(frag.sql.trim() == expected.trim());
    
}