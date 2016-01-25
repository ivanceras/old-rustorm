extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use uuid::Uuid;

use rustorm::dao::{Dao, IsDao};
use rustorm::pool::ManagedPool;
use rustorm::em::EntityManager;
use rustorm::table::{Table, Column};
use rustorm::table::IsTable;
use rustorm::dao::Type;
use rustorm::dao::Value;
use rustorm::query::Operand;


#[derive(Debug, Clone)]
pub struct Product {
    pub product_id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

impl IsDao for Product{
    fn from_dao(dao: &Dao) -> Self {
        Product {
            product_id: dao.get("product_id"),
            name: dao.get("name"),
            description: dao.get_opt("description"),
        }
    }
    fn to_dao(&self) -> Dao {
        let mut dao = Dao::new();
        dao.set("product_id", &self.product_id);
        dao.set("name", &self.name);
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
            columns: vec![
                Column{
                    name:"product_id".to_string(),
                    data_type: Type::Uuid,
                    db_data_type:"uuid".to_string(),
                    is_primary:true, is_unique:false, not_null:true, is_inherited:false,
                    default:Some(Operand::Value(Value::String("uuid_generate_v4()".to_owned()))),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"name".to_string(),
                    data_type: Type::String,
                    db_data_type:"character varying".to_string(),
                    is_primary:false, is_unique:false, not_null:true, is_inherited:false,
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"description".to_string(),
                    data_type: Type::String,
                    db_data_type:"character varying".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true,
                    default:None,
                    comment:None,
                    foreign:None,
                },
            ],
            is_view: false,
        }
    }
}


fn main() {
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v8";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();
    let em = EntityManager::new(db.as_ref());

    let pid = Uuid::parse_str("6db712e6-cc50-4c3a-8269-451c98ace5ad").unwrap();
    let prod: Product = em.get_exact(&pid).unwrap();

    println!("{}  {}  {:?}", prod.product_id, prod.name, prod.description);
    //pool.release(db);
}
