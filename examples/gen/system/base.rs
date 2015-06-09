use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use uuid::Uuid;
use rustorm::table::IsTable;
use rustorm::table::Column;
use rustorm::table::Foreign;
use rustorm::table::Table;



///
/// Base table contains the creation and modification status of a record
///
#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct Base {
    /// db data type: uuid
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// db data type: timestamp with time zone
    pub created:DateTime<UTC>,
    /// db data type: uuid
    pub created_by:Option<Uuid>,
    /// db data type: uuid
    pub organization_id:Option<Uuid>,
    /// priority of saving data and eviction
    /// db data type: numeric
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// db data type: timestamp with time zone
    pub updated:DateTime<UTC>,
    /// db data type: uuid
    pub updated_by:Option<Uuid>,
}


impl IsTable for Base{

    fn table()->Table{
    
        Table{
            schema:"system".to_string(),
            name:"base".to_string(),
            parent_table:None,
            sub_table:Some(vec!["record".to_string(),"product_availability".to_string(),"product_category".to_string(),"product_photo".to_string(),"product_review".to_string(),]),
            comment:Some("Base table contains the creation and modification status of a record".to_string()),
            columns:
            vec![
                Column{
                    name:"organization_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"client_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"created".to_string(),
                    data_type:"DateTime<UTC>".to_string(),
                    db_data_type:"timestamp with time zone".to_string(),
                    is_primary:false, is_unique:false, not_null:true, is_inherited:false, 
                    default:Some("now()".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"created_by".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"updated".to_string(),
                    data_type:"DateTime<UTC>".to_string(),
                    db_data_type:"timestamp with time zone".to_string(),
                    is_primary:false, is_unique:false, not_null:true, is_inherited:false, 
                    default:Some("now()".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"updated_by".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"priority".to_string(),
                    data_type:"f64".to_string(),
                    db_data_type:"numeric".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:Some("priority of saving data and eviction".to_string()),
                    foreign:None,
                },
            ],
        }
    }
}