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
use rustorm::query::ToColumnName;
use rustorm::query::Filter;
use rustorm::query::HasEquality;

use bazaar::product;

mod bazaar{
	use rustorm::table::{IsTable, Table};
    pub fn product() -> Table {
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
	pub mod product{
		use rustorm::query::ColumnName;

		pub fn name()->ColumnName{
			ColumnName::from_str("product.name")
		}	
		pub fn description()->ColumnName{
			ColumnName::from_str("product.description")
		}	
	}
}


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

fn main() {
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v8";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();

	let mut query = Query::SELECT_ALL();
		
					query.FROM(&bazaar::product)
					.WHERE(product::name.EQ(&"GTX660 Ti videocard") & 
					product::description.NEQ(&"no description"));

    let prod: Result<Product,_> =  query
                            .collect_one(db.as_ref());

	let sql = query.debug_build(db.as_ref());
	println!("sql: {}", sql);

	let prod = prod.unwrap();
    println!("{}  {}  {:?}",
             prod.product_id,
             prod.name.unwrap(),
             prod.description);
}
