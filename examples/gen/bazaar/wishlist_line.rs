use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use uuid::Uuid;
use gen::bazaar::Wishlist;
use rustorm::dao::Dao;
use rustorm::dao::IsDao;
use rustorm::dao::DaoResult;
use std::collections::BTreeMap;
use rustorm::table::IsTable;
use rustorm::table::Column;
use rustorm::table::Foreign;
use rustorm::table::Table;



#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct WishlistLine {
    /// primary
    /// not nullable 
    /// db data type: uuid
    pub wishlist_line_id:Uuid,
    /// default: false
    /// db data type: boolean
    pub added_to_cart:Option<bool>,
    /// db data type: numeric
    pub price_momentary:Option<f64>,
    /// db data type: uuid
    pub product_id:Option<Uuid>,
    /// db data type: uuid
    pub wishlist_id:Option<Uuid>,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    /// db data type: boolean
    pub active:bool,
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
    /// db data type: character varying
    pub description:Option<String>,
    /// --inherited-- 
    /// db data type: text
    pub help:Option<String>,
    /// --inherited-- 
    /// db data type: character varying
    pub name:Option<String>,
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
    pub wishlist: Option<Wishlist>,
}



impl IsDao for WishlistLine{
    fn from_dao(dao:&Dao)->Self{
        WishlistLine{
            organization_id: dao.get_opt("organization_id"),
            client_id: dao.get_opt("client_id"),
            created: dao.get("created"),
            created_by: dao.get_opt("created_by"),
            updated: dao.get("updated"),
            updated_by: dao.get_opt("updated_by"),
            priority: dao.get_opt("priority"),
            name: dao.get_opt("name"),
            description: dao.get_opt("description"),
            help: dao.get_opt("help"),
            active: dao.get("active"),
            wishlist_id: dao.get_opt("wishlist_id"),
            price_momentary: dao.get_opt("price_momentary"),
            product_id: dao.get_opt("product_id"),
            added_to_cart: dao.get_opt("added_to_cart"),
            wishlist_line_id: dao.get("wishlist_line_id"),
            wishlist: None,
        }
    }
}

impl IsTable for WishlistLine{

    fn table()->Table{
    
        Table{
            schema:"bazaar".to_string(),
            name:"wishlist_line".to_string(),
            parent_table:Some("record".to_string()),
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
                    name:"name".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"character varying".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
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
                Column{
                    name:"help".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"text".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"active".to_string(),
                    data_type:"bool".to_string(),
                    db_data_type:"boolean".to_string(),
                    is_primary:false, is_unique:false, not_null:true, is_inherited:true, 
                    default:Some("true".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"wishlist_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:Some(
                        Foreign{
                            schema:"bazaar".to_string(),
                            table:"wishlist".to_string(),
                            column:"wishlist_id".to_string(),
                        }),
                },
                Column{
                    name:"price_momentary".to_string(),
                    data_type:"f64".to_string(),
                    db_data_type:"numeric".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"product_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"added_to_cart".to_string(),
                    data_type:"bool".to_string(),
                    db_data_type:"boolean".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:Some("false".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"wishlist_line_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:true, is_unique:false, not_null:true, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
            ],
        }
    }
}