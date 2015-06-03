use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use uuid::Uuid;


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct Country {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    /// db data type: uuid
    pub country_id:Uuid,
    /// db data type: character varying
    pub code:Option<String>,
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
    pub createdby:Option<Uuid>,
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
    pub updatedby:Option<Uuid>,
    /// has many
    pub currency:Option<Vec<Currency>>,
}


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct Currency {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    /// db data type: uuid
    pub currency_id:Uuid,
    /// which country uses this currency
    /// db data type: uuid
    pub country_id:Option<Uuid>,
    /// db data type: character varying
    pub symbol:Option<String>,
    /// db data type: character varying
    pub unicode:Option<String>,
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
    pub createdby:Option<Uuid>,
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
    pub updatedby:Option<Uuid>,
    /// has one
    pub country:Option<Country>,
    /// has many
    pub exchange_rate:Option<Vec<ExchangeRate>>,
    /// has many
    pub product:Option<Vec<Product>>,
}


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct ExchangeRate {
    /// primary
    /// this will be referred when processing payments with different currencies
    /// default: uuid_generate_v4()
    /// not nullable 
    /// db data type: uuid
    pub exchange_rate_id:Uuid,
    /// db data type: numeric
    pub exchange_rate:Option<f64>,
    /// db data type: uuid
    pub from_currency:Option<Uuid>,
    /// db data type: uuid
    pub to_currency:Option<Uuid>,
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
    pub createdby:Option<Uuid>,
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
    pub updatedby:Option<Uuid>,
    /// has one
    pub from:Option<Currency>,
    /// has one
    pub to:Option<Currency>,
}
