extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use uuid::Uuid;

use rustorm::query::Query;
use rustorm::query::Equality;

use rustorm::pool::ManagedPool;
use rustorm::database::Database;
use rustorm::dao::{IsDao, Dao};
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

/// on a webserver this will be the main thread, where it instantiate
/// the connection pool in the entirety of the application
/// when a request in made, a thread is spawned for that request
/// with an access to the a connection pool
fn main() {
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v8";
    let pool = ManagedPool::init(url, 1);
    match pool {
        Ok(pool) => {
            let db = pool.connect();

            match db {
                Ok(db) => {
                    show_product(db.as_ref());//borrow a database
                }
                Err(e) => {
                    println!("Unable to connect to database {}", e);
                }
            }
        }
        Err(_) => {
            panic!("Unable to connect to database")
        }
    }
}

/// a dispatched controller with an accesss to a database reference
fn show_product(db: &Database) {
    let prod: Product = Query::select_all()
                            .from_table("bazaar.product")
                            .filter("name", Equality::EQ, &"GTX660 Ti videocard")
                            .collect_one(db)
                            .unwrap();

    println!("{}  {}  {:?}",
             prod.product_id,
             prod.name.unwrap(),
             prod.description);

}
