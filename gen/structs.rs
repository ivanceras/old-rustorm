use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use uuid::Uuid;


///
/// @Address
///
#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct Address {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    /// db data type: uuid
    pub address_id:Uuid,
    /// distance is computed on the fly using the formula in sql, this is here to provide a property on the Models to store the value
    /// db data type: numeric
    pub distance:Option<f64>,
    /// db data type: numeric
    pub latitude:Option<f64>,
    /// db data type: numeric
    pub longitude:Option<f64>,
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
    pub user_info:Option<Vec<UserInfo>>,
}


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct ApiKey {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    /// db data type: uuid
    pub api_key_id:Uuid,
    /// not nullable 
    /// db data type: character varying
    pub api_key:String,
    /// not nullable 
    /// db data type: uuid
    pub user_id:Uuid,
    /// db data type: timestamp with time zone
    pub valid_starting:Option<DateTime<UTC>>,
    /// db data type: timestamp with time zone
    pub valid_until:Option<DateTime<UTC>>,
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
    pub user:Option<Users>,
}


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
    pub createdby:Option<Uuid>,
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
    pub updatedby:Option<Uuid>,
}


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct Cart {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    /// db data type: uuid
    pub cart_id:Uuid,
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
    pub cart_line:Option<Vec<CartLine>>,
}


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct CartLine {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    /// db data type: uuid
    pub cart_line_id:Uuid,
    /// db data type: uuid
    pub cart_id:Option<Uuid>,
    /// db data type: uuid
    pub product_id:Option<Uuid>,
    /// db data type: numeric
    pub qty:Option<f64>,
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
    pub cart:Option<Cart>,
}


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct Category {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    /// db data type: uuid
    pub category_id:Uuid,
    /// unique
    /// --inherited-- 
    /// db data type: character varying
    pub name:Option<String>,
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
    /// has many, indirect referring table, derived from linker table: product_category
    pub product:Option<Vec<Product>>,
}


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct Client {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    /// --inherited-- 
    /// db data type: uuid
    pub client_id:Uuid,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    /// db data type: boolean
    pub active:bool,
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
}


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


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct Invoice {
    /// default: uuid_generate_v4()
    /// db data type: uuid
    pub invoice_id:Option<Uuid>,
    /// db data type: boolean
    pub is_paid:Option<bool>,
    /// db data type: uuid
    pub order_id:Option<Uuid>,
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
}


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct OrderLine {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    /// db data type: uuid
    pub order_line_id:Uuid,
    /// db data type: numeric
    pub discount:Option<f64>,
    /// db data type: numeric
    pub freight_amt:Option<f64>,
    /// db data type: uuid
    pub order_id:Option<Uuid>,
    /// db data type: numeric
    pub price_momentary:Option<f64>,
    /// db data type: uuid
    pub product_id:Option<Uuid>,
    /// db data type: numeric
    pub qty_ordered:Option<f64>,
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
    pub order:Option<Orders>,
}


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
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
    pub order_line:Option<Vec<OrderLine>>,
}


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct Organization {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    /// --inherited-- 
    /// db data type: uuid
    pub organization_id:Uuid,
    /// db data type: uuid
    pub address_id:Option<Uuid>,
    /// db data type: character varying
    pub landmark:Option<String>,
    /// db data type: uuid
    pub parent_organization_id:Option<Uuid>,
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
    /// has one, self referential
    pub parent:Option<Box<Organization>>,
    /// has many
    pub organization:Option<Vec<Organization>>,
}


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct Photo {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    /// db data type: uuid
    pub photo_id:Uuid,
    /// The base64 encoding of the image, which can be stored in the database
    /// db data type: character varying
    pub data:Option<String>,
    /// db data type: integer
    pub seq_no:Option<i32>,
    /// The online version of the photo, could be hosted in cdn somewhere else, to avoid payloads in the system. The online photo can be cached by creating a base64 encoding, then storing it in the local db
    /// db data type: character varying
    pub url:Option<String>,
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
    /// has many, indirect referring table, derived from linker table: product_photo
    pub product:Option<Vec<Product>>,
    /// has many
    pub photo_sizes:Option<Vec<PhotoSizes>>,
    /// has many
    pub user_info:Option<Vec<UserInfo>>,
}


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct PhotoSizes {
    /// primary
    /// not nullable 
    /// db data type: uuid
    pub photo_id:Uuid,
    /// primary
    /// not nullable 
    /// db data type: uuid
    pub photo_size_id:Uuid,
    /// The base64 encoding of this photo, optimized to this size
    /// db data type: character varying
    pub data:Option<String>,
    /// db data type: integer
    pub height:Option<i32>,
    /// db data type: character varying
    pub url:Option<String>,
    /// db data type: integer
    pub width:Option<i32>,
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
    pub photo:Option<Photo>,
}


///
/// This will be exposed as an @Api, including @Table(users, category, product_availability, photo)
///
#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct Product {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    /// db data type: uuid
    pub product_id:Uuid,
    /// barcode if scanning the product, conflict can happen, expect to return matching list of products using the barcode
    /// db data type: character varying
    pub barcode:Option<String>,
    /// db data type: uuid
    pub currency_id:Option<Uuid>,
    /// {color:"red",
    /// dimension:"10x20x30",
    /// dimensionUnit:"mm",
    /// weight:"4",
    /// weightUnit:"kg"
    /// }
    /// db data type: json
    pub info:Option<String>,
    /// default: false
    /// db data type: boolean
    pub is_service:Option<bool>,
    /// Whom this product belongs, since createdby can be someone else create the product list in behalf of the owner of the product
    /// db data type: uuid
    pub owner_id:Option<Uuid>,
    /// db data type: uuid
    pub parent_product_id:Option<Uuid>,
    /// db data type: numeric
    pub price:Option<f64>,
    /// @Sequence can be used to do alternate ordering of the values, when alphetical or time can not be used
    /// db data type: integer
    pub seq_no:Option<i32>,
    /// db data type: json
    pub tags:Option<String>,
    /// db data type: character varying
    pub unit:Option<String>,
    /// Applicable to services, usually services has an upfront fee
    /// default: 0.00
    /// db data type: numeric
    pub upfront_fee:Option<f64>,
    /// default: false
    /// db data type: boolean
    pub use_parent_price:Option<bool>,
    /// @Active
    /// default: true
    /// not nullable 
    /// --inherited-- 
    /// db data type: boolean
    pub active:bool,
    /// @Value(users.client_id) The client_id of the user creating this records
    /// --inherited-- 
    /// db data type: uuid
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    /// db data type: timestamp with time zone
    pub created:DateTime<UTC>,
    /// @Value(users.user_id)
    /// --inherited-- 
    /// db data type: uuid
    pub createdby:Option<Uuid>,
    /// @DisplayLength(100) When building a UI for this field
    /// @MaxLength(200) Do not go over 200 character on this one
    /// --inherited-- 
    /// db data type: character varying
    pub description:Option<String>,
    /// --inherited-- 
    /// db data type: text
    pub help:Option<String>,
    /// This is @Required it has @DisplayLength(50) - 50 character in display length a @MinLength(1) and @MaxLength(100) - Do not go over 100 characters or else the system will throw a ValueTooLong exception
    /// can also be express with @Length(1-100)
    /// --inherited-- 
    /// db data type: character varying
    pub name:Option<String>,
    /// @Value(users.user_id) , which means the value will be set with the users.user_id value
    /// 
    /// @Where(users.active=true)
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
    /// @Value(users.user_id)
    /// --inherited-- 
    /// db data type: uuid
    pub updatedby:Option<Uuid>,
    /// has one
    pub owner:Option<Users>,
    /// has one
    pub currency:Option<Currency>,
    /// has one, extension table
    pub product_availability:Option<Box<ProductAvailability>>,
    /// has many, indirect referring table, derived from linker table: product_category
    pub category:Option<Vec<Category>>,
    /// has many, indirect referring table, derived from linker table: product_photo
    pub photo:Option<Vec<Photo>>,
    /// has many, indirect referring table, derived from linker table: product_review
    pub review:Option<Vec<Review>>,
}


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
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
    pub close_time:Option<DateTime<UTC>>,
    /// db data type: time with time zone
    pub open_time:Option<DateTime<UTC>>,
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
    pub createdby:Option<Uuid>,
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
    pub product:Option<Product>,
}


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct ProductCategory {
    /// primary
    /// not nullable 
    /// db data type: uuid
    pub category_id:Uuid,
    /// primary
    /// not nullable 
    /// db data type: uuid
    pub product_id:Uuid,
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
    pub product:Option<Product>,
    /// has one
    pub category:Option<Category>,
}


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct ProductPhoto {
    /// primary
    /// not nullable 
    /// db data type: uuid
    pub photo_id:Uuid,
    /// primary
    /// not nullable 
    /// db data type: uuid
    pub product_id:Uuid,
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
    pub product:Option<Product>,
    /// has one
    pub photo:Option<Photo>,
}


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct ProductReview {
    /// primary
    /// not nullable 
    /// db data type: uuid
    pub product_id:Uuid,
    /// primary
    /// not nullable 
    /// db data type: uuid
    pub review_id:Uuid,
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
    pub product:Option<Product>,
    /// has one
    pub review:Option<Review>,
}


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
    pub createdby:Option<Uuid>,
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
}


///
/// Reviews of buyers from the sellers and the sellers' products
///
#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct Review {
    /// primary
    /// not nullable 
    /// db data type: uuid
    pub review_id:Uuid,
    /// db data type: boolean
    pub approved:Option<bool>,
    /// the user id who approves the review
    /// db data type: uuid
    pub approvedby:Option<Uuid>,
    /// The statement of the review
    /// db data type: character varying
    pub comment:Option<String>,
    /// rating 1 to 5, 5 is the highest
    /// db data type: integer
    pub rating:Option<i32>,
    /// db data type: uuid
    pub user_id:Option<Uuid>,
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
    /// has many, indirect referring table, derived from linker table: product_review
    pub product:Option<Vec<Product>>,
    /// has many, indirect referring table, derived from linker table: user_review
    pub users:Option<Vec<Users>>,
}


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct Settings {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    /// db data type: uuid
    pub settings_id:Uuid,
    /// Use metric system as unit, if false, use english system
    /// default: true
    /// db data type: boolean
    pub use_metric:Option<bool>,
    /// db data type: uuid
    pub user_id:Option<Uuid>,
    /// db data type: json
    pub value:Option<String>,
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
    pub user:Option<Users>,
}


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct UserInfo {
    /// primary
    /// not nullable 
    /// db data type: uuid
    pub user_id:Uuid,
    /// db data type: uuid
    pub address_id:Option<Uuid>,
    /// db data type: character varying
    pub current_location:Option<String>,
    /// db data type: character varying
    pub displayname:Option<String>,
    /// db data type: uuid
    pub photo_id:Option<Uuid>,
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
    pub user:Option<Users>,
    /// has one
    pub address:Option<Address>,
    /// has one
    pub photo:Option<Photo>,
}


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct UserLocation {
    /// primary
    /// not nullable 
    /// db data type: uuid
    pub user_id:Uuid,
    /// db data type: numeric
    pub accuracy:Option<f64>,
    /// user can anonymize their location by setting loose accuracy
    /// db data type: numeric
    pub set_accuracy:Option<f64>,
    /// db data type: numeric
    pub set_latitude:Option<f64>,
    /// db data type: numeric
    pub set_longitude:Option<f64>,
    /// db data type: numeric
    pub true_latitude:Option<f64>,
    /// db data type: numeric
    pub true_longitude:Option<f64>,
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
    pub user:Option<Users>,
}


///
/// Reviews of the seller by the user
///
#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct UserReview {
    /// primary
    /// not nullable 
    /// db data type: uuid
    pub review_id:Uuid,
    /// primary
    /// The user id of the seller being reviewed
    /// not nullable 
    /// db data type: uuid
    pub user_id:Uuid,
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
    pub user:Option<Users>,
    /// has one
    pub review:Option<Review>,
}


///
/// This are @Users, will be used for @Login
///
#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
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
    /// has one, extension table
    pub user_info:Option<Box<UserInfo>>,
    /// has one, extension table
    pub user_location:Option<Box<UserLocation>>,
    /// has many, indirect referring table, derived from linker table: user_review
    pub review:Option<Vec<Review>>,
    /// has many
    pub api_key:Option<Vec<ApiKey>>,
    /// has many
    pub product:Option<Vec<Product>>,
    /// has many
    pub settings:Option<Vec<Settings>>,
}


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct Wishlist {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    /// db data type: uuid
    pub wishlist_id:Uuid,
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
    pub wishlist_line:Option<Vec<WishlistLine>>,
}


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
    pub wishlist:Option<Wishlist>,
}
