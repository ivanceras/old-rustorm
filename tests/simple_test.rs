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



#[derive(Debug, Clone)]
pub struct Product {
    pub product_id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl IsDao for Product{
    fn from_dao(dao: &Dao) -> Self {
        Product {
            product_id: dao.get("product_id"),
            name: dao.get_opt("name"),
            description: dao.get_opt("description"),
        }
    }

    fn to_dao(&self) -> Dao {
        let mut dao = Dao::new();
        dao.set("product_id", &self.product_id);
        match self.name {
            Some(ref _value) => dao.set("name", _value),
            None => dao.set_null("name"),
        }
        match self.description {
            Some(ref _value) => dao.set("description", _value),
            None => dao.set_null("description"),
        }
        dao
    }
}

impl IsTable for Product{
    fn table() -> Table {
        Table {
            schema: Some("bazaar".to_string()),
            name: "product".to_string(),
            parent_table: None,
            sub_table: vec![],
            comment: None,
            columns: vec![],
            is_view: false,
        }
    }
}

#[test]
fn test_simple_query() {
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v8";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();

    let prod: Product = Query::select_all()
                            .from_table("bazaar.product")
                            .filter("name", Equality::EQ, &"GTX660 Ti videocard")
                            .collect_one(db.as_ref())
                            .unwrap();

    println!("{}  {}  {:?}",
             prod.product_id,
             prod.name.unwrap(),
             prod.description);
}
