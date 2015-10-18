extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use uuid::Uuid;
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;

use rustorm::dao::{Dao, IsDao};
use rustorm::pool::ManagedPool;
#[allow(unused_imports)]
use rustorm::database::Database;
use rustorm::table::IsTable;
use rustorm::table::Column;
use rustorm::table::Table;
use rustorm::table::Foreign;




fn main() {
    let url = "sqlite:///file1.db";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();
    db.as_ddl().create_table(&Product::table());
}



///
/// This will be exposed as an @Api, including @Table(users, category, product_availability, photo)
///
#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug, Clone)]
pub struct Product {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable
    /// db data type: uuid
    pub product_id: Uuid,
    /// barcode if scanning the product, conflict can happen, expect to return matching list of products using the barcode
    /// db data type: character varying
    pub barcode: Option<String>,
    /// db data type: uuid
    pub currency_id: Option<Uuid>,
    /// {color:"red",
    /// dimension:"10x20x30",
    /// dimensionUnit:"mm",
    /// weight:"4",
    /// weightUnit:"kg"
    /// }
    /// db data type: json
    pub info: Option<String>,
    /// default: false
    /// db data type: boolean
    pub is_service: Option<bool>,
    /// Whom this product belongs, since created_by can be someone else create the product list in behalf of the owner of the product
    /// db data type: uuid
    pub owner_id: Option<Uuid>,
    /// db data type: uuid
    pub parent_product_id: Option<Uuid>,
    /// db data type: numeric
    pub price: Option<f64>,
    /// @Sequence can be used to do alternate ordering of the values, when alphetical or time can not be used
    /// db data type: integer
    pub seq_no: Option<i32>,
    /// db data type: json
    pub tags: Option<String>,
    /// db data type: character varying
    pub unit: Option<String>,
    /// Applicable to services, usually services has an upfront fee
    /// default: 0.00
    /// db data type: numeric
    pub upfront_fee: Option<f64>,
    /// default: false
    /// db data type: boolean
    pub use_parent_price: Option<bool>,
    /// @Active
    /// default: true
    /// not nullable
    /// --inherited--
    /// db data type: boolean
    pub active: bool,
    /// @Value(users.client_id) The client_id of the user creating this records
    /// --inherited--
    /// db data type: uuid
    pub client_id: Option<Uuid>,
    /// default: now()
    /// not nullable
    /// --inherited--
    /// db data type: timestamp with time zone
    pub created: DateTime<UTC>,
    /// @Value(users.user_id)
    /// --inherited--
    /// db data type: uuid
    pub created_by: Option<Uuid>,
    /// @DisplayLength(100) When building a UI for this field
    /// @MaxLength(200) Do not go over 200 character on this one
    /// --inherited--
    /// db data type: character varying
    pub description: Option<String>,
    /// --inherited--
    /// db data type: text
    pub help: Option<String>,
    /// This is @Required it has @DisplayLength(50) - 50 character in display length a @MinLength(1) and @MaxLength(100) - Do not go over 100 characters or else the system will throw a ValueTooLong exception
    /// can also be express with @Length(1-100)
    /// --inherited--
    /// db data type: character varying
    pub name: Option<String>,
    /// @Value(users.user_id) , which means the value will be set with the users.user_id value
    ///
    /// @Where(users.active=true)
    /// --inherited--
    /// db data type: uuid
    pub organization_id: Option<Uuid>,
    /// --inherited--
    /// db data type: numeric
    pub priority: Option<f64>,
    /// default: now()
    /// not nullable
    /// --inherited--
    /// db data type: timestamp with time zone
    pub updated: DateTime<UTC>,
    /// @Value(users.user_id)
    /// --inherited--
    /// db data type: uuid
    pub updated_by: Option<Uuid>,

}



impl IsDao for Product{
    fn from_dao(dao: &Dao) -> Self {
        Product {
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
            product_id: dao.get("product_id"),
            parent_product_id: dao.get_opt("parent_product_id"),
            is_service: dao.get_opt("is_service"),
            price: dao.get_opt("price"),
            use_parent_price: dao.get_opt("use_parent_price"),
            unit: dao.get_opt("unit"),
            tags: dao.get_opt("tags"),
            info: dao.get_opt("info"),
            seq_no: dao.get_opt("seq_no"),
            upfront_fee: dao.get_opt("upfront_fee"),
            barcode: dao.get_opt("barcode"),
            owner_id: dao.get_opt("owner_id"),
            currency_id: dao.get_opt("currency_id"),
        }
    }

    fn to_dao(&self) -> Dao {
        let mut dao = Dao::new();
        match self.organization_id {
            Some(ref _value) => dao.set("organization_id", _value),
            None => dao.set_null("organization_id"),
        }
        match self.client_id {
            Some(ref _value) => dao.set("client_id", _value),
            None => dao.set_null("client_id"),
        }
        dao.set("created", &self.created);
        match self.created_by {
            Some(ref _value) => dao.set("created_by", _value),
            None => dao.set_null("created_by"),
        }
        dao.set("updated", &self.updated);
        match self.updated_by {
            Some(ref _value) => dao.set("updated_by", _value),
            None => dao.set_null("updated_by"),
        }
        match self.priority {
            Some(ref _value) => dao.set("priority", _value),
            None => dao.set_null("priority"),
        }
        match self.name {
            Some(ref _value) => dao.set("name", _value),
            None => dao.set_null("name"),
        }
        match self.description {
            Some(ref _value) => dao.set("description", _value),
            None => dao.set_null("description"),
        }
        match self.help {
            Some(ref _value) => dao.set("help", _value),
            None => dao.set_null("help"),
        }
        dao.set("active", &self.active);
        dao.set("product_id", &self.product_id);
        match self.parent_product_id {
            Some(ref _value) => dao.set("parent_product_id", _value),
            None => dao.set_null("parent_product_id"),
        }
        match self.is_service {
            Some(ref _value) => dao.set("is_service", _value),
            None => dao.set_null("is_service"),
        }
        match self.price {
            Some(ref _value) => dao.set("price", _value),
            None => dao.set_null("price"),
        }
        match self.use_parent_price {
            Some(ref _value) => dao.set("use_parent_price", _value),
            None => dao.set_null("use_parent_price"),
        }
        match self.unit {
            Some(ref _value) => dao.set("unit", _value),
            None => dao.set_null("unit"),
        }
        match self.tags {
            Some(ref _value) => dao.set("tags", _value),
            None => dao.set_null("tags"),
        }
        match self.info {
            Some(ref _value) => dao.set("info", _value),
            None => dao.set_null("info"),
        }
        match self.seq_no {
            Some(ref _value) => dao.set("seq_no", _value),
            None => dao.set_null("seq_no"),
        }
        match self.upfront_fee {
            Some(ref _value) => dao.set("upfront_fee", _value),
            None => dao.set_null("upfront_fee"),
        }
        match self.barcode {
            Some(ref _value) => dao.set("barcode", _value),
            None => dao.set_null("barcode"),
        }
        match self.owner_id {
            Some(ref _value) => dao.set("owner_id", _value),
            None => dao.set_null("owner_id"),
        }
        match self.currency_id {
            Some(ref _value) => dao.set("currency_id", _value),
            None => dao.set_null("currency_id"),
        }
        dao
    }
}

impl IsTable for Product{

    fn table() -> Table {

        Table{
            schema:"bazaar".to_string(),
            name:"product".to_string(),
            parent_table:Some("record".to_string()),
            sub_table:vec![],
            comment:Some("This will be exposed as an @Api, including @Table(users, category, product_availability, photo)".to_string()),
            columns:
            vec![
                Column{
                    name:"organization_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true,
                    default:None,
                    comment:Some("@Value(users.user_id) , which means the value will be set with the users.user_id value\n\n@Where(users.active=true)".to_string()),
                    foreign:None,
                },
                Column{
                    name:"client_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true,
                    default:None,
                    comment:Some("@Value(users.client_id) The client_id of the user creating this records".to_string()),
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
                    comment:Some("@Value(users.user_id)".to_string()),
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
                    comment:Some("@Value(users.user_id)".to_string()),
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
                    comment:Some("This is @Required it has @DisplayLength(50) - 50 character in display length a @MinLength(1) and @MaxLength(100) - Do not go over 100 characters or else the system will throw a ValueTooLong exception\ncan also be express with @Length(1-100)".to_string()),
                    foreign:None,
                },
                Column{
                    name:"description".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"character varying".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:true,
                    default:None,
                    comment:Some("@DisplayLength(100) When building a UI for this field\n@MaxLength(200) Do not go over 200 character on this one".to_string()),
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
                    name:"product_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:true, is_unique:false, not_null:true, is_inherited:false,
                    default:Some("uuid_generate_v4()".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"parent_product_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false,
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"is_service".to_string(),
                    data_type:"bool".to_string(),
                    db_data_type:"boolean".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false,
                    default:Some("false".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"price".to_string(),
                    data_type:"f64".to_string(),
                    db_data_type:"numeric".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false,
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"use_parent_price".to_string(),
                    data_type:"bool".to_string(),
                    db_data_type:"boolean".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false,
                    default:Some("false".to_string()),
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"unit".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"character varying".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false,
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"tags".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"json".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false,
                    default:None,
                    comment:None,
                    foreign:None,
                },
                Column{
                    name:"info".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"json".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false,
                    default:None,
                    comment:Some("{color:\"red\",\ndimension:\"10x20x30\",\ndimensionUnit:\"mm\",\nweight:\"4\",\nweightUnit:\"kg\"\n}".to_string()),
                    foreign:None,
                },
                Column{
                    name:"seq_no".to_string(),
                    data_type:"i32".to_string(),
                    db_data_type:"integer".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false,
                    default:None,
                    comment:Some("@Sequence can be used to do alternate ordering of the values, when alphetical or time can not be used".to_string()),
                    foreign:None,
                },
                Column{
                    name:"upfront_fee".to_string(),
                    data_type:"f64".to_string(),
                    db_data_type:"numeric".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false,
                    default:Some("0.00".to_string()),
                    comment:Some("Applicable to services, usually services has an upfront fee".to_string()),
                    foreign:None,
                },
                Column{
                    name:"barcode".to_string(),
                    data_type:"String".to_string(),
                    db_data_type:"character varying".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false,
                    default:None,
                    comment:Some("barcode if scanning the product, conflict can happen, expect to return matching list of products using the barcode".to_string()),
                    foreign:None,
                },
                Column{
                    name:"owner_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false,
                    default:None,
                    comment:Some("Whom this product belongs, since created_by can be someone else create the product list in behalf of the owner of the product".to_string()),
                    foreign:Some(
                        Foreign{
                            schema:"bazaar".to_string(),
                            table:"users".to_string(),
                            column:"user_id".to_string(),
                        }),
                },
                Column{
                    name:"currency_id".to_string(),
                    data_type:"Uuid".to_string(),
                    db_data_type:"uuid".to_string(),
                    is_primary:false, is_unique:false, not_null:false, is_inherited:false,
                    default:None,
                    comment:None,
                    foreign:Some(
                        Foreign{
                            schema:"payment".to_string(),
                            table:"currency".to_string(),
                            column:"currency_id".to_string(),
                        }),
                },
            ],
            is_view: false
        }
    }
}
