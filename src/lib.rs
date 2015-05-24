extern crate postgres;
extern crate rustc_serialize;
extern crate uuid;
extern crate chrono;
extern crate regex;



use table::Table;
use table::Column;

pub mod em;
pub mod filter;
pub mod query;
pub mod types;
pub mod database;
pub mod meta;
pub mod join;
pub mod db;
pub mod table;
pub mod writer;
pub mod gen;
pub mod codegen;
pub mod window;



#[test]
pub fn test_equal_column(){
    println!("testing here..");
    let c1 = Column{name:"product_id".to_string(),
            data_type:"Uuid".to_string(),
            is_primary:true,
            is_unique:false,
            default:None,
            comment:None,
            not_null:false,
            foreign:None,
            is_inherited:false,
        };
    let c2 = Column{name:"description".to_string(),
            data_type:"String".to_string(),
            is_primary:true,
            is_unique:false,
            default:None,
            comment:None,
            not_null:false,
            foreign:None,
            is_inherited:false,
        };
    let v1 = vec![&c1, &c2];
    let v2 = vec![&c2, &c1];
    assert_eq!(v1,v2);
}
