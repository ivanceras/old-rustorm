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
use rustorm::table::{Table, IsTable, Column};
use rustorm::database::DatabaseDDL;

#[derive(Debug, Clone)]
struct Category {
    pub name:Option<String>,
    pub description:Option<String>,
}

impl IsDao for Category{
    fn from_dao(dao:&Dao)->Self{
        Category{
            name: dao.get_opt("name"),
            description: dao.get_opt("description"),
        }
    }
}



impl IsTable for Category{

    fn table()->Table{
    
        Table{
            schema:"bazaar".to_string(),
            name:"category".to_string(),
            parent_table: None,
            sub_table:vec![],
            comment:None,
            columns:
            vec![
                Column{
                    name:"name".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"character varying".to_string(),
                    is_primary:false, is_unique:true, not_null:false, is_inherited:true, 
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
    let url = "sqlite:///file.db";
    let db = pool.get_db_with_url(&url).unwrap();
    let result = db.as_ddl().create_table(&Category::table());
    
    let mut query = Query::insert();
    query.into_(&Category::table());
    query.set("name", &"sqlite");
    query.set("description", &"This is inserted into sqlite in memory database");
    query.execute(db.as_ref());
    
    let category: Category = Query::select()
        .columns(vec!["name","description"])
        .from(&Category::table())
        .collect_one(db.as_ref());
    
        println!("Category: {:?}", category);
    
}
