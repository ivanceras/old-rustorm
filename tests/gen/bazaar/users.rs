//! WARNING: THIS FILE IS GENERATED, DERIVED FROM TABLE bazaar.users, DO NOT EDIT

use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use uuid::Uuid;
use gen::bazaar::ApiKey;
use gen::bazaar::Product;
use gen::bazaar::Review;
use gen::bazaar::Settings;
use gen::bazaar::UserInfo;
use gen::bazaar::UserLocation;
use rustorm::dao::Dao;
use rustorm::dao::IsDao;
use rustorm::table::IsTable;
use rustorm::table::Column;
use rustorm::table::Table;



///
/// This are @Users, will be used for @Login
///
#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug, Clone)]
pub struct Users {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    /// db data type: uuid
    pub user_id:Uuid,
    /// @Email
    /// db data type: character varying
    pub email:Option<String>,
    /// The users' @Password will be check against the value, while you can also specify hashing alogrithm used of the value @Hash(SHA256), or just @SHA256.
    /// 
    /// SHA512, CLEAR_TEXT, MD5 can also be used.
    /// @Length(8-50)
    /// @DisplayLength(20)
    /// db data type: character varying
    pub password:Option<String>,
    /// @Username
    /// @DisplayLength(20)
    /// @Length(2-100)
    /// db data type: character varying
    pub username:Option<String>,
    /// @Active
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

    /// has one, extension table
    pub info: Option<Box<UserInfo>>,
    /// has one, extension table
    pub location: Option<Box<UserLocation>>,
    /// has many
    pub api_key: Vec<ApiKey>,
    /// has many
    pub product: Vec<Product>,
    /// has many
    pub settings: Vec<Settings>,
    /// has many, indirect
    pub review: Vec<Review>,
}


// Generated columns for easier development of dynamic queries without sacrificing wrong spelling of column names

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static organization_id: &'static str = "users.organization_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static client_id: &'static str = "users.client_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created: &'static str = "users.created";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static created_by: &'static str = "users.created_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated: &'static str = "users.updated";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static updated_by: &'static str = "users.updated_by";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static priority: &'static str = "users.priority";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static name: &'static str = "users.name";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static description: &'static str = "users.description";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static help: &'static str = "users.help";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static active: &'static str = "users.active";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static user_id: &'static str = "users.user_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static username: &'static str = "users.username";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static password: &'static str = "users.password";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static email: &'static str = "users.email";


impl IsDao for Users{
    fn from_dao(dao:&Dao)->Self{
        Users{
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
            user_id: dao.get("user_id"),
            username: dao.get_opt("username"),
            password: dao.get_opt("password"),
            email: dao.get_opt("email"),
            info: None,
            location: None,
            api_key: vec![],
            product: vec![],
            settings: vec![],
            review: vec![],
        }
    }
}

impl IsTable for Users{

    fn table()->Table{
    
        Table{
            schema:"bazaar".to_string(),
            name:"users".to_string(),
            parent_table:Some("record".to_string()),
            sub_table:vec![],
            comment:Some("This are @Users, will be used for @Login".to_string()),
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
                    comment:Some("@Active".to_string()),
                    foreign:None,
                },
                Column{
                    name:"user_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:true, is_unique:false, not_null:true, is_inherited:false, 
                    default:Some("uuid_generate_v4()".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"username".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"character varying".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:Some("@Username\n@DisplayLength(20)\n@Length(2-100)".to_string()),
                    foreign:None,
                },
                Column{
                    name:"password".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"character varying".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:Some("The users' @Password will be check against the value, while you can also specify hashing alogrithm used of the value @Hash(SHA256), or just @SHA256.\n\nSHA512, CLEAR_TEXT, MD5 can also be used.\n@Length(8-50)\n@DisplayLength(20)".to_string()),
                    foreign:None,
                },
                Column{
                    name:"email".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"character varying".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false, 
                    default:None,
                    comment:Some("@Email".to_string()),
                    foreign:None,
                },
            ],
        }
    }
}