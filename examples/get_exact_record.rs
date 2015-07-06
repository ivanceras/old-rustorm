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
use rustorm::pool::Pool;
use rustorm::em::EntityManager;
use rustorm::table::{Table,Column};
use rustorm::table::IsTable;



#[derive(Debug, Clone)]
pub struct Product {
    pub product_id:Uuid,
    pub name:String,
    pub description:Option<String>,
}

impl IsDao for Product{
    fn from_dao(dao:&Dao)->Self{
        Product{
            product_id: dao.get("product_id"),
            name: dao.get("name"),
            description: dao.get_opt("description"),
        }
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
            columns:
            vec![
                Column{
                    name:"product_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:true, is_unique:false, not_null:true, is_inherited:false, 
                    default:Some("uuid_generate_v4()".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"name".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"character varying".to_string(),
                    is_primary:false, is_unique:false, not_null:true, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"description".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"character varying".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
            ],
        }
    }
}


fn main(){
    let mut pool = Pool::init();
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    let db = pool.from_url(&url).unwrap();
    let em = EntityManager::new(db.as_ref());
    
    let pid = Uuid::parse_str("6db712e6-cc50-4c3a-8269-451c98ace5ad").unwrap();
    let prod: Product = em.get_exact(&pid);
    
    println!("{}  {}  {:?}", prod.product_id, prod.name, prod.description);
    //pool.release(db);
}