//! WARNING: THIS FILE IS GENERATED, DERIVED FROM TABLE bazaar.orders, DO NOT EDIT

use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use uuid::Uuid;
use gen::bazaar::OrderLine;
use rustorm::dao::Dao;
use rustorm::dao::IsDao;
use rustorm::table::IsTable;
use rustorm::table::Column;
use rustorm::table::Table;



#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug, Clone)]
pub struct Orders {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    /// db data type: uuid
    pub order_id:Uuid,
    /// db data type: numeric
    pub amount_refunded:Option<f64>,
    /// db data type: numeric
    pub amount_tendered:Option<f64>,
    /// The cart from which this order was created from
    /// db data type: uuid
    pub cart_id:Option<Uuid>,
    /// default: 0.00
    /// db data type: numeric
    pub charges_amount:Option<f64>,
    /// For recognization purposes, this is the name shown to the seller
    /// db data type: character varying
    pub customer_name:Option<String>,
    /// db data type: timestamp with time zone
    pub date_approved:Option<DateTime<UTC>>,
    /// db data type: timestamp with time zone
    pub date_invoiced:Option<DateTime<UTC>>,
    /// default: now()
    /// db data type: timestamp with time zone
    pub date_ordered:Option<DateTime<UTC>>,
    /// db data type: numeric
    pub grand_total_amount:Option<f64>,
    /// if the order from the buyer is approved by the seller
    /// default: false
    /// db data type: boolean
    pub is_approved:Option<bool>,
    /// determined whether the order has been confirmed by the person who ordered it
    /// default: false
    /// db data type: boolean
    pub is_confirmed:Option<bool>,
    /// default: false
    /// db data type: boolean
    pub is_invoiced:Option<bool>,
    /// default: true
    /// db data type: boolean
    pub is_tax_included:Option<bool>,
    /// default: false
    /// db data type: boolean
    pub processed:Option<bool>,
    /// default: false
    /// db data type: boolean
    pub processing:Option<bool>,
    /// db data type: integer
    pub total_items:Option<i32>,
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

    /// has many
    pub order_line: Vec<OrderLine>,
}


// Generated columns for easier development of dynamic queries without sacrificing wrong spelling of column names

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static organization_id: &'static str = "orders.organization_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static client_id: &'static str = "orders.client_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created: &'static str = "orders.created";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created_by: &'static str = "orders.created_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated: &'static str = "orders.updated";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated_by: &'static str = "orders.updated_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static priority: &'static str = "orders.priority";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static name: &'static str = "orders.name";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static description: &'static str = "orders.description";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static help: &'static str = "orders.help";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static active: &'static str = "orders.active";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static order_id: &'static str = "orders.order_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static customer_name: &'static str = "orders.customer_name";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static total_items: &'static str = "orders.total_items";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static grand_total_amount: &'static str = "orders.grand_total_amount";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static charges_amount: &'static str = "orders.charges_amount";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static processing: &'static str = "orders.processing";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static processed: &'static str = "orders.processed";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static is_confirmed: &'static str = "orders.is_confirmed";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static is_tax_included: &'static str = "orders.is_tax_included";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static date_ordered: &'static str = "orders.date_ordered";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static is_invoiced: &'static str = "orders.is_invoiced";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static date_invoiced: &'static str = "orders.date_invoiced";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static is_approved: &'static str = "orders.is_approved";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static date_approved: &'static str = "orders.date_approved";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static amount_tendered: &'static str = "orders.amount_tendered";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static amount_refunded: &'static str = "orders.amount_refunded";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static cart_id: &'static str = "orders.cart_id";


impl IsDao for Orders{
    fn from_dao(dao:&Dao)->Self{
        Orders{
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
            order_id: dao.get("order_id"),
            customer_name: dao.get_opt("customer_name"),
            total_items: dao.get_opt("total_items"),
            grand_total_amount: dao.get_opt("grand_total_amount"),
            charges_amount: dao.get_opt("charges_amount"),
            processing: dao.get_opt("processing"),
            processed: dao.get_opt("processed"),
            is_confirmed: dao.get_opt("is_confirmed"),
            is_tax_included: dao.get_opt("is_tax_included"),
            date_ordered: dao.get_opt("date_ordered"),
            is_invoiced: dao.get_opt("is_invoiced"),
            date_invoiced: dao.get_opt("date_invoiced"),
            is_approved: dao.get_opt("is_approved"),
            date_approved: dao.get_opt("date_approved"),
            amount_tendered: dao.get_opt("amount_tendered"),
            amount_refunded: dao.get_opt("amount_refunded"),
            cart_id: dao.get_opt("cart_id"),
            order_line: vec![],
        }
    }
}

impl IsTable for Orders{

    fn table()->Table{
    
        Table{
            schema:"bazaar".to_string(),
            name:"orders".to_string(),
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
                    name:"order_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:true, is_unique:false, not_null:true, is_inherited:false, 
                    default:Some("uuid_generate_v4()".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"customer_name".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"character varying".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:Some("For recognization purposes, this is the name shown to the seller".to_string()),
                    foreign:None,
                },
                Column{
                    name:"total_items".to_string(),
                    data_type:"i32".to_string(),
                    db_data_type:"integer".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"grand_total_amount".to_string(),
                    data_type:"f64".to_string(),
                    db_data_type:"numeric".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"charges_amount".to_string(),
                    data_type:"f64".to_string(),
                    db_data_type:"numeric".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:Some("0.00".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"processing".to_string(),
                    data_type:"bool".to_string(),
                    db_data_type:"boolean".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:Some("false".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"processed".to_string(),
                    data_type:"bool".to_string(),
                    db_data_type:"boolean".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:Some("false".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"is_confirmed".to_string(),
                    data_type:"bool".to_string(),
                    db_data_type:"boolean".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:Some("false".to_string()),
                    comment:Some("determined whether the order has been confirmed by the person who ordered it".to_string()),
                    foreign:None,
                },
                Column{
                    name:"is_tax_included".to_string(),
                    data_type:"bool".to_string(),
                    db_data_type:"boolean".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:Some("true".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"date_ordered".to_string(),
                    data_type:"DateTime<UTC>".to_string(),
                    db_data_type:"timestamp with time zone".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:Some("now()".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"is_invoiced".to_string(),
                    data_type:"bool".to_string(),
                    db_data_type:"boolean".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:Some("false".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"date_invoiced".to_string(),
                    data_type:"DateTime<UTC>".to_string(),
                    db_data_type:"timestamp with time zone".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"is_approved".to_string(),
                    data_type:"bool".to_string(),
                    db_data_type:"boolean".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:Some("false".to_string()),
                    comment:Some("if the order from the buyer is approved by the seller".to_string()),
                    foreign:None,
                },
                Column{
                    name:"date_approved".to_string(),
                    data_type:"DateTime<UTC>".to_string(),
                    db_data_type:"timestamp with time zone".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"amount_tendered".to_string(),
                    data_type:"f64".to_string(),
                    db_data_type:"numeric".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"amount_refunded".to_string(),
                    data_type:"f64".to_string(),
                    db_data_type:"numeric".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"cart_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:Some("The cart from which this order was created from".to_string()),
                    foreign:None,
                },
            ],
        }
    }
}