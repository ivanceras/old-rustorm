//! WARNING: THIS FILE IS GENERATED, DERIVED FROM TABLE bazaar.product_availability, DO NOT EDIT

use chrono::datetime::DateTime;
use chrono::naive::time::NaiveTime;
use chrono::offset::utc::UTC;
use uuid::Uuid;
use gen::bazaar::Product;
use rustorm::dao::Dao;
use rustorm::dao::IsDao;
use rustorm::table::IsTable;
use rustorm::table::Column;
use rustorm::table::Table;
use rustorm::table::Foreign;



#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug, Clone)]
pub struct ProductAvailability {
    /// primary
    /// not nullable 
    /// db data type: uuid
    pub product_id:Uuid,
    /// db data type: boolean
    pub always_available:Option<bool>,
    /// db data type: boolean
    pub available:Option<bool>,
    /// {"Mon", "Tue", "Wed", "Thur", "Fri", "Sat", "Sun"}
    /// db data type: json
    pub available_day:Option<String>,
    /// db data type: timestamp with time zone
    pub available_from:Option<DateTime<UTC>>,
    /// db data type: timestamp with time zone
    pub available_until:Option<DateTime<UTC>>,
    /// db data type: time with time zone
    pub close_time:Option<NaiveTime>,
    /// db data type: time with time zone
    pub open_time:Option<NaiveTime>,
    /// default: 1
    /// db data type: numeric
    pub stocks:Option<f64>,
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

    /// has one
    pub product: Option<Product>,
}


// Generated columns for easier development of dynamic queries without sacrificing wrong spelling of column names

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static organization_id: &'static str = "product_availability.organization_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static client_id: &'static str = "product_availability.client_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created: &'static str = "product_availability.created";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created_by: &'static str = "product_availability.created_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated: &'static str = "product_availability.updated";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated_by: &'static str = "product_availability.updated_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static priority: &'static str = "product_availability.priority";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static product_id: &'static str = "product_availability.product_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static available: &'static str = "product_availability.available";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static always_available: &'static str = "product_availability.always_available";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static stocks: &'static str = "product_availability.stocks";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static available_from: &'static str = "product_availability.available_from";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static available_until: &'static str = "product_availability.available_until";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static available_day: &'static str = "product_availability.available_day";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static open_time: &'static str = "product_availability.open_time";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static close_time: &'static str = "product_availability.close_time";


impl IsDao for ProductAvailability{
    fn from_dao(dao:&Dao)->Self{
        ProductAvailability{
            organization_id: dao.get_opt("organization_id"),
            client_id: dao.get_opt("client_id"),
            created: dao.get("created"),
            created_by: dao.get_opt("created_by"),
            updated: dao.get("updated"),
            updated_by: dao.get_opt("updated_by"),
            priority: dao.get_opt("priority"),
            product_id: dao.get("product_id"),
            available: dao.get_opt("available"),
            always_available: dao.get_opt("always_available"),
            stocks: dao.get_opt("stocks"),
            available_from: dao.get_opt("available_from"),
            available_until: dao.get_opt("available_until"),
            available_day: dao.get_opt("available_day"),
            open_time: dao.get_opt("open_time"),
            close_time: dao.get_opt("close_time"),
            product: None,
        }
    }
}

impl IsTable for ProductAvailability{

    fn table()->Table{
    
        Table{
            schema:"bazaar".to_string(),
            name:"product_availability".to_string(),
            parent_table:Some("base".to_string()),
            sub_table:vec![],
            comment:None,
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
                    name:"product_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:true, is_unique:false, not_null:true, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:Some(
                        Foreign{
                            schema:"bazaar".to_string(),
                            table:"product".to_string(),
                            column:"product_id".to_string(),
                        }),
                },
                Column{
                    name:"available".to_string(),
                    data_type:"bool".to_string(),
                    db_data_type:"boolean".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"always_available".to_string(),
                    data_type:"bool".to_string(),
                    db_data_type:"boolean".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"stocks".to_string(),
                    data_type:"f64".to_string(),
                    db_data_type:"numeric".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:Some("1".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"available_from".to_string(),
                    data_type:"DateTime<UTC>".to_string(),
                    db_data_type:"timestamp with time zone".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"available_until".to_string(),
                    data_type:"DateTime<UTC>".to_string(),
                    db_data_type:"timestamp with time zone".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"available_day".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"json".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:Some("{\"Mon\", \"Tue\", \"Wed\", \"Thur\", \"Fri\", \"Sat\", \"Sun\"}".to_string()),
                    foreign:None,
                },
                Column{
                    name:"open_time".to_string(),
                    data_type:"NaiveTime".to_string(),
                    db_data_type:"time with time zone".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"close_time".to_string(),
                    data_type:"NaiveTime".to_string(),
                    db_data_type:"time with time zone".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
            ],
        }
    }
}