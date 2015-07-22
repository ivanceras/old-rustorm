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
use rustorm::table::{IsTable,Table};


#[derive(Debug, Clone)]
pub struct Product {
    pub product_id:Uuid,
    pub name:Option<String>,
    pub description:Option<String>,
}

impl IsDao for Product{
    fn from_dao(dao:&Dao)->Self{
        Product{
            product_id: dao.get("product_id"),
            name: dao.get_opt("name"),
            description: dao.get_opt("description"),
        }
    }
    fn to_dao(&self)->Dao{
        let mut dao = Dao::new();
        dao.set("product_id", &self.product_id);
        match self.name{
            Some(ref _value) => dao.set("name", _value),
            None => dao.set_null("name"),
        };
        match self.description{
            Some(ref _value) => dao.set("description", _value),
            None => dao.set_null("description"),
        };
        dao
    }
}

impl IsTable for Product{
    
    fn table()->Table{
        Table{
            schema:"bazaar".to_string(),
            name:"product".to_string(),
            parent_table:None,
            sub_table:vec![],
            comment:None,
            columns:vec![]
        }
    }
}


fn main(){
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    let mut pool = ManagedPool::init(&url, 1);
    let mut db = pool.connect().unwrap();
    
    let products: Vec<Product> = Query::select_all()
            .from_table("bazaar.product")
            .collect(db.as_mut());
    
    for prod in products{
        let name = prod.name.unwrap();
        let desc = match prod.description{
                        Some(desc) => desc,
                        None => "".to_string()
                    };
        println!("{}  {}  {:?}", prod.product_id, name, desc);
    }
}