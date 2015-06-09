use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use uuid::Uuid;
use rustorm::table::IsTable;
use rustorm::table::Column;
use rustorm::table::Foreign;
use rustorm::table::Table;



///
/// All User table should inherit from this one
///
#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct Record {
    /// @Active
    /// default: true
    /// not nullable 
    /// db data type: boolean
    pub active:bool,
    /// db data type: character varying
    pub description:Option<String>,
    /// db data type: text
    pub help:Option<String>,
    /// db data type: character varying
    pub name:Option<String>,
    /// --inherited-- 
    /// db data type: uuid
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    /// db data type: timestamp with time zone
    pub created:DateTime<UTC>,
    /// --inherited-- 
    /// db data type: uuid
    pub created_by:Option<Uuid>,
    /// --inherited-- 
    /// db data type: uuid
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    /// db data type: numeric
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    /// db data type: timestamp with time zone
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    /// db data type: uuid
    pub updated_by:Option<Uuid>,
}


impl IsTable for Record{

    fn table()->Table{
    
        Table{
            schema:"system".to_string(),
            name:"record".to_string(),
            parent_table:Some("base".to_string()),
            sub_table:Some(vec!["address".to_string(),"api_key".to_string(),"cart".to_string(),"cart_line".to_string(),"category".to_string(),"client".to_string(),"invoice".to_string(),"order_line".to_string(),"orders".to_string(),"organization".to_string(),"photo".to_string(),"photo_sizes".to_string(),"product".to_string(),"review".to_string(),"settings".to_string(),"user_info".to_string(),"user_location".to_string(),"user_review".to_string(),"users".to_string(),"wishlist".to_string(),"wishlist_line".to_string(),"country".to_string(),"currency".to_string(),"exchange_rate".to_string(),]),
            comment:Some("All User table should inherit from this one".to_string()),
            columns:
            vec![
                Column{
                    name:"organization_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"client_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"created".to_string(),
                    data_type:"DateTime<UTC>".to_string(),
                    db_data_type:"timestamp with time zone".to_string(),
                    is_primary:false, is_unique:false, not_null:true, is_inherited:true, 
                    default:Some("now()".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"created_by".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"updated".to_string(),
                    data_type:"DateTime<UTC>".to_string(),
                    db_data_type:"timestamp with time zone".to_string(),
                    is_primary:false, is_unique:false, not_null:true, is_inherited:true, 
                    default:Some("now()".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"updated_by".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"priority".to_string(),
                    data_type:"f64".to_string(),
                    db_data_type:"numeric".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"name".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"character varying".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"description".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"character varying".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"help".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"text".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"active".to_string(),
                    data_type:"bool".to_string(),
                    db_data_type:"boolean".to_string(),
                    is_primary:false, is_unique:false, not_null:true, is_inherited:false, 
                    default:Some("true".to_string()),
                    comment:Some("@Active".to_string()),
                    foreign:None,
                },
            ],
        }
    }
}