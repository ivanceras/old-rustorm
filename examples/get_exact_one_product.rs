extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use rustorm::db::postgres::Postgres;
use uuid::Uuid;

use rustorm::em::EntityManager;
use gen::bazaar::Product;

mod gen;
 

fn main(){
    let pg= Postgres::connect_with_url("postgres://postgres:p0stgr3s@localhost/bazaar_v6").unwrap();
    let em = EntityManager::new(&pg);
    let pid = Uuid::parse_str("6db712e6-cc50-4c3a-8269-451c98ace5ad").unwrap();
    let prod: Product = em.get_exact(&pid);
    println!("{}  {}  {:?}", prod.product_id, prod.name.unwrap(), prod.description);
}
