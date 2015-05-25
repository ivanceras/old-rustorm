extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use rustorm::gen::structs::Address;
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use rustc_serialize::json::Json;
use uuid::Uuid;
use rustorm::table::IsTable;
use rustorm::table::Table;
use rustorm::table::Column;
use rustorm::gen::is_table::*;


fn retrieve<T:IsTable>(dao: T){
    let sql = format!("SELECT * FROM {}.{}", T::schema(), T::table_name());
    println!("sql: {}", sql);
}


fn main(){
     let addr = Address {
        address_id:Uuid::new_v4(),
        distance:None,
        latitude:None,
        longitude:None,
        active:true,
        client_id:None,
        created:UTC::now(),
        createdby:None,
        description:None,
        help:None,
        name:None,
        organization_id:None,
        priority:None,
        updated:UTC::now(),
        updatedby:None,
        user_info:None,
    };
     
    retrieve(addr);
}


fn table()->Table{
   Table{
        schema:"bazaar".to_string(),
        name:"product".to_string(),
        parent_table:Some("record".to_string()),
        sub_table:None,
        comment:None,
        columns:
                vec![
                Column{name:"product_id".to_string(), 
                	data_type:"Uuid".to_string(), 
                	db_data_type:"uuid".to_string(), 
                	is_primary:true, 
                	is_unique:false, 
                	default:Some("uuid_generate_v4()".to_string()), 
                	comment:None, 
                	not_null:true, 
                	foreign:None, 
                	is_inherited:false
                	},
                Column{name:"barcode".to_string(), data_type:"String".to_string(), db_data_type:"character varying".to_string(), is_primary:false, is_unique:false, default:None, comment:Some("barcode if scanning the product, conflict can happen, expect to return matching list of products using the barcode".to_string()), not_null:false, foreign:None, is_inherited:false},
                ],
                
    }
}    