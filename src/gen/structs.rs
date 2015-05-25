use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use rustc_serialize::json::Json;
use uuid::Uuid;


///
/// @Address
///
#[derive(Debug)]
pub struct Address {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    pub address_id:Uuid,
    /// distance is computed on the fly using the formula in sql, this is here to provide a property on the Models to store the value
    pub distance:Option<f64>,
    pub latitude:Option<f64>,
    pub longitude:Option<f64>,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    pub active:bool,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub description:Option<String>,
    /// --inherited-- 
    pub help:Option<String>,
    /// --inherited-- 
    pub name:Option<String>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
    /// has many
    pub user_info:Option<Vec<UserInfo>>,
}


#[derive(Debug)]
pub struct ApiKey {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    pub api_key_id:Uuid,
    /// not nullable 
    pub api_key:String,
    /// not nullable 
    pub user_id:Uuid,
    pub valid_starting:Option<DateTime<UTC>>,
    pub valid_until:Option<DateTime<UTC>>,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    pub active:bool,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub description:Option<String>,
    /// --inherited-- 
    pub help:Option<String>,
    /// --inherited-- 
    pub name:Option<String>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
    /// has one
    pub user:Option<Users>,
}


///
/// Base table contains the creation and modification status of a record
///
#[derive(Debug)]
pub struct Base {
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    pub created:DateTime<UTC>,
    pub createdby:Option<Uuid>,
    pub organization_id:Option<Uuid>,
    /// priority of saving data and eviction
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    pub updated:DateTime<UTC>,
    pub updatedby:Option<Uuid>,
}


#[derive(Debug)]
pub struct Cart {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    pub cart_id:Uuid,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    pub active:bool,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub description:Option<String>,
    /// --inherited-- 
    pub help:Option<String>,
    /// --inherited-- 
    pub name:Option<String>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
    /// has many
    pub cart_line:Option<Vec<CartLine>>,
}


#[derive(Debug)]
pub struct CartLine {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    pub cart_line_id:Uuid,
    pub cart_id:Option<Uuid>,
    pub product_id:Option<Uuid>,
    pub qty:Option<f64>,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    pub active:bool,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub description:Option<String>,
    /// --inherited-- 
    pub help:Option<String>,
    /// --inherited-- 
    pub name:Option<String>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
    /// has one
    pub cart:Option<Cart>,
}


#[derive(Debug)]
pub struct Category {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    pub category_id:Uuid,
    /// unique
    /// --inherited-- 
    pub name:Option<String>,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    pub active:bool,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub description:Option<String>,
    /// --inherited-- 
    pub help:Option<String>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
    /// has many, indirect referring table, derived from linker table: product_category
    pub product:Option<Vec<Product>>,
}


#[derive(Debug)]
pub struct Client {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    /// --inherited-- 
    pub client_id:Uuid,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    pub active:bool,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub description:Option<String>,
    /// --inherited-- 
    pub help:Option<String>,
    /// --inherited-- 
    pub name:Option<String>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
}


#[derive(Debug)]
pub struct Country {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    pub country_id:Uuid,
    pub code:Option<String>,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    pub active:bool,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub description:Option<String>,
    /// --inherited-- 
    pub help:Option<String>,
    /// --inherited-- 
    pub name:Option<String>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
    /// has many
    pub currency:Option<Vec<Currency>>,
}


#[derive(Debug)]
pub struct Currency {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    pub currency_id:Uuid,
    /// which country uses this currency
    pub country_id:Option<Uuid>,
    pub symbol:Option<String>,
    pub unicode:Option<String>,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    pub active:bool,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub description:Option<String>,
    /// --inherited-- 
    pub help:Option<String>,
    /// --inherited-- 
    pub name:Option<String>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
    /// has one
    pub country:Option<Country>,
    /// has many
    pub exchange_rate:Option<Vec<ExchangeRate>>,
    /// has many
    pub product:Option<Vec<Product>>,
}


#[derive(Debug)]
pub struct ExchangeRate {
    /// primary
    /// this will be referred when processing payments with different currencies
    /// default: uuid_generate_v4()
    /// not nullable 
    pub exchange_rate_id:Uuid,
    pub exchange_rate:Option<f64>,
    pub from_currency:Option<Uuid>,
    pub to_currency:Option<Uuid>,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    pub active:bool,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub description:Option<String>,
    /// --inherited-- 
    pub help:Option<String>,
    /// --inherited-- 
    pub name:Option<String>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
    /// has one
    pub from:Option<Currency>,
    /// has one
    pub to:Option<Currency>,
}


#[derive(Debug)]
pub struct Invoice {
    /// default: uuid_generate_v4()
    pub invoice_id:Option<Uuid>,
    pub is_paid:Option<bool>,
    pub order_id:Option<Uuid>,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    pub active:bool,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub description:Option<String>,
    /// --inherited-- 
    pub help:Option<String>,
    /// --inherited-- 
    pub name:Option<String>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
}


#[derive(Debug)]
pub struct OrderLine {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    pub order_line_id:Uuid,
    pub discount:Option<f64>,
    pub freight_amt:Option<f64>,
    pub order_id:Option<Uuid>,
    pub price_momentary:Option<f64>,
    pub product_id:Option<Uuid>,
    pub qty_ordered:Option<f64>,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    pub active:bool,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub description:Option<String>,
    /// --inherited-- 
    pub help:Option<String>,
    /// --inherited-- 
    pub name:Option<String>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
    /// has one
    pub order:Option<Orders>,
}


#[derive(Debug)]
pub struct Orders {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    pub order_id:Uuid,
    pub amount_refunded:Option<f64>,
    pub amount_tendered:Option<f64>,
    /// The cart from which this order was created from
    pub cart_id:Option<Uuid>,
    /// default: 0.00
    pub charges_amount:Option<f64>,
    /// For recognization purposes, this is the name shown to the seller
    pub customer_name:Option<String>,
    pub date_approved:Option<DateTime<UTC>>,
    pub date_invoiced:Option<DateTime<UTC>>,
    /// default: now()
    pub date_ordered:Option<DateTime<UTC>>,
    pub grand_total_amount:Option<f64>,
    /// if the order from the buyer is approved by the seller
    /// default: false
    pub is_approved:Option<bool>,
    /// determined whether the order has been confirmed by the person who ordered it
    /// default: false
    pub is_confirmed:Option<bool>,
    /// default: false
    pub is_invoiced:Option<bool>,
    /// default: true
    pub is_tax_included:Option<bool>,
    /// default: false
    pub processed:Option<bool>,
    /// default: false
    pub processing:Option<bool>,
    pub total_items:Option<i32>,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    pub active:bool,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub description:Option<String>,
    /// --inherited-- 
    pub help:Option<String>,
    /// --inherited-- 
    pub name:Option<String>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
    /// has many
    pub order_line:Option<Vec<OrderLine>>,
}


#[derive(Debug)]
pub struct Organization {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    /// --inherited-- 
    pub organization_id:Uuid,
    pub address_id:Option<Uuid>,
    pub landmark:Option<String>,
    pub parent_organization_id:Option<Uuid>,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    pub active:bool,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub description:Option<String>,
    /// --inherited-- 
    pub help:Option<String>,
    /// --inherited-- 
    pub name:Option<String>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
    /// has one, self referential
    pub parent:Option<Box<Organization>>,
    /// has many
    pub organization:Option<Vec<Organization>>,
}


#[derive(Debug)]
pub struct Photo {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    pub photo_id:Uuid,
    /// The base64 encoding of the image, which can be stored in the database
    pub data:Option<String>,
    pub seq_no:Option<i32>,
    /// The online version of the photo, could be hosted in cdn somewhere else, to avoid payloads in the system. The online photo can be cached by creating a base64 encoding, then storing it in the local db
    pub url:Option<String>,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    pub active:bool,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub description:Option<String>,
    /// --inherited-- 
    pub help:Option<String>,
    /// --inherited-- 
    pub name:Option<String>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
    /// has one, extension table
    pub photo_sizes:Option<Box<PhotoSizes>>,
    /// has many, indirect referring table, derived from linker table: product_photo
    pub product:Option<Vec<Product>>,
    /// has many
    pub user_info:Option<Vec<UserInfo>>,
}


#[derive(Debug)]
pub struct PhotoSizes {
    /// primary
    /// not nullable 
    pub photo_id:Uuid,
    /// primary
    /// not nullable 
    pub photo_size_id:Uuid,
    /// The base64 encoding of this photo, optimized to this size
    pub data:Option<String>,
    pub height:Option<i32>,
    pub url:Option<String>,
    pub width:Option<i32>,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    pub active:bool,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub description:Option<String>,
    /// --inherited-- 
    pub help:Option<String>,
    /// --inherited-- 
    pub name:Option<String>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
    /// has one
    pub photo:Option<Photo>,
}


///
/// This will be exposed as an @Api, including @Table(users, category, product_availability, photo)
///
#[derive(Debug)]
pub struct Product {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    pub product_id:Uuid,
    /// barcode if scanning the product, conflict can happen, expect to return matching list of products using the barcode
    pub barcode:Option<String>,
    pub currency_id:Option<Uuid>,
    /// {color:"red",
    /// dimension:"10x20x30",
    /// dimensionUnit:"mm",
    /// weight:"4",
    /// weightUnit:"kg"
    /// }
    pub info:Option<Json>,
    /// default: false
    pub is_service:Option<bool>,
    /// Whom this product belongs, since createdby can be someone else create the product list in behalf of the owner of the product
    pub owner_id:Option<Uuid>,
    pub parent_product_id:Option<Uuid>,
    pub price:Option<f64>,
    /// @Sequence can be used to do alternate ordering of the values, when alphetical or time can not be used
    pub seq_no:Option<i32>,
    pub tags:Option<Json>,
    pub unit:Option<String>,
    /// Applicable to services, usually services has an upfront fee
    /// default: 0.00
    pub upfront_fee:Option<f64>,
    /// default: false
    pub use_parent_price:Option<bool>,
    /// @Active
    /// default: true
    /// not nullable 
    /// --inherited-- 
    pub active:bool,
    /// @Value(users.client_id) The client_id of the user creating this records
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// @Value(users.user_id)
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// @DisplayLength(100) When building a UI for this field
    /// @MaxLength(200) Do not go over 200 character on this one
    /// --inherited-- 
    pub description:Option<String>,
    /// --inherited-- 
    pub help:Option<String>,
    /// This is @Required it has @DisplayLength(50) - 50 character in display length a @MinLength(1) and @MaxLength(100) - Do not go over 100 characters or else the system will throw a ValueTooLong exception
    /// can also be express with @Length(1-100)
    /// --inherited-- 
    pub name:Option<String>,
    /// @Value(users.user_id) , which means the value will be set with the users.user_id value
    /// 
    /// @Where(users.active=true)
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// @Value(users.user_id)
    /// --inherited-- 
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


#[derive(Debug)]
pub struct ProductAvailability {
    /// primary
    /// not nullable 
    pub product_id:Uuid,
    pub always_available:Option<bool>,
    pub available:Option<bool>,
    /// {"Mon", "Tue", "Wed", "Thur", "Fri", "Sat", "Sun"}
    pub available_day:Option<Json>,
    pub available_from:Option<DateTime<UTC>>,
    pub available_until:Option<DateTime<UTC>>,
    pub close_time:Option<DateTime<UTC>>,
    pub open_time:Option<DateTime<UTC>>,
    /// default: 1
    pub stocks:Option<f64>,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
    /// has one
    pub product:Option<Product>,
}


#[derive(Debug)]
pub struct ProductCategory {
    /// primary
    /// not nullable 
    pub category_id:Uuid,
    /// primary
    /// not nullable 
    pub product_id:Uuid,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
    /// has one
    pub product:Option<Product>,
    /// has one
    pub category:Option<Category>,
}


#[derive(Debug)]
pub struct ProductPhoto {
    /// primary
    /// not nullable 
    pub photo_id:Uuid,
    /// primary
    /// not nullable 
    pub product_id:Uuid,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
    /// has one
    pub product:Option<Product>,
    /// has one
    pub photo:Option<Photo>,
}


#[derive(Debug)]
pub struct ProductReview {
    /// primary
    /// not nullable 
    pub product_id:Uuid,
    /// primary
    /// not nullable 
    pub review_id:Uuid,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
    /// has one
    pub product:Option<Product>,
    /// has one
    pub review:Option<Review>,
}


///
/// All User table should inherit from this one
///
#[derive(Debug)]
pub struct Record {
    /// @Active
    /// default: true
    /// not nullable 
    pub active:bool,
    pub description:Option<String>,
    pub help:Option<String>,
    pub name:Option<String>,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
}


///
/// Reviews of buyers from the sellers and the sellers' products
///
#[derive(Debug)]
pub struct Review {
    /// primary
    /// not nullable 
    pub review_id:Uuid,
    pub approved:Option<bool>,
    /// the user id who approves the review
    pub approvedby:Option<Uuid>,
    /// The statement of the review
    pub comment:Option<String>,
    /// rating 1 to 5, 5 is the highest
    pub rating:Option<i32>,
    pub user_id:Option<Uuid>,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    pub active:bool,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub description:Option<String>,
    /// --inherited-- 
    pub help:Option<String>,
    /// --inherited-- 
    pub name:Option<String>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
    /// has many, indirect referring table, derived from linker table: product_review
    pub product:Option<Vec<Product>>,
    /// has many, indirect referring table, derived from linker table: user_review
    pub users:Option<Vec<Users>>,
}


#[derive(Debug)]
pub struct Settings {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    pub settings_id:Uuid,
    /// Use metric system as unit, if false, use english system
    /// default: true
    pub use_metric:Option<bool>,
    pub user_id:Option<Uuid>,
    pub value:Option<Json>,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    pub active:bool,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub description:Option<String>,
    /// --inherited-- 
    pub help:Option<String>,
    /// --inherited-- 
    pub name:Option<String>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
    /// has one
    pub user:Option<Users>,
}


#[derive(Debug)]
pub struct UserInfo {
    /// primary
    /// not nullable 
    pub user_id:Uuid,
    pub address_id:Option<Uuid>,
    pub current_location:Option<String>,
    pub displayname:Option<String>,
    pub photo_id:Option<Uuid>,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    pub active:bool,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub description:Option<String>,
    /// --inherited-- 
    pub help:Option<String>,
    /// --inherited-- 
    pub name:Option<String>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
    /// has one
    pub user:Option<Users>,
    /// has one
    pub address:Option<Address>,
    /// has one
    pub photo:Option<Photo>,
}


#[derive(Debug)]
pub struct UserLocation {
    /// primary
    /// not nullable 
    pub user_id:Uuid,
    pub accuracy:Option<f64>,
    /// user can anonymize their location by setting loose accuracy
    pub set_accuracy:Option<f64>,
    pub set_latitude:Option<f64>,
    pub set_longitude:Option<f64>,
    pub true_latitude:Option<f64>,
    pub true_longitude:Option<f64>,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    pub active:bool,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub description:Option<String>,
    /// --inherited-- 
    pub help:Option<String>,
    /// --inherited-- 
    pub name:Option<String>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
    /// has one
    pub user:Option<Users>,
}


///
/// Reviews of the seller by the user
///
#[derive(Debug)]
pub struct UserReview {
    /// primary
    /// not nullable 
    pub review_id:Uuid,
    /// primary
    /// The user id of the seller being reviewed
    /// not nullable 
    pub user_id:Uuid,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    pub active:bool,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub description:Option<String>,
    /// --inherited-- 
    pub help:Option<String>,
    /// --inherited-- 
    pub name:Option<String>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
    /// has one
    pub user:Option<Users>,
    /// has one
    pub review:Option<Review>,
}


///
/// This are @Users, will be used for @Login
///
#[derive(Debug)]
pub struct Users {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    pub user_id:Uuid,
    /// @Email
    pub email:Option<String>,
    /// The users' @Password will be check against the value, while you can also specify hashing alogrithm used of the value @Hash(SHA256), or just @SHA256.
    /// 
    /// SHA512, CLEAR_TEXT, MD5 can also be used.
    /// @Length(8-50)
    /// @DisplayLength(20)
    pub password:Option<String>,
    /// @Username
    /// @DisplayLength(20)
    /// @Length(2-100)
    pub username:Option<String>,
    /// @Active
    /// default: true
    /// not nullable 
    /// --inherited-- 
    pub active:bool,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub description:Option<String>,
    /// --inherited-- 
    pub help:Option<String>,
    /// --inherited-- 
    pub name:Option<String>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
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


#[derive(Debug)]
pub struct Wishlist {
    /// primary
    /// default: uuid_generate_v4()
    /// not nullable 
    pub wishlist_id:Uuid,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    pub active:bool,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub description:Option<String>,
    /// --inherited-- 
    pub help:Option<String>,
    /// --inherited-- 
    pub name:Option<String>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
    /// has many
    pub wishlist_line:Option<Vec<WishlistLine>>,
}


#[derive(Debug)]
pub struct WishlistLine {
    /// primary
    /// not nullable 
    pub wishlist_line_id:Uuid,
    /// default: false
    pub added_to_cart:Option<bool>,
    pub price_momentary:Option<f64>,
    pub product_id:Option<Uuid>,
    pub wishlist_id:Option<Uuid>,
    /// default: true
    /// not nullable 
    /// --inherited-- 
    pub active:bool,
    /// --inherited-- 
    pub client_id:Option<Uuid>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub created:DateTime<UTC>,
    /// --inherited-- 
    pub createdby:Option<Uuid>,
    /// --inherited-- 
    pub description:Option<String>,
    /// --inherited-- 
    pub help:Option<String>,
    /// --inherited-- 
    pub name:Option<String>,
    /// --inherited-- 
    pub organization_id:Option<Uuid>,
    /// --inherited-- 
    pub priority:Option<f64>,
    /// default: now()
    /// not nullable 
    /// --inherited-- 
    pub updated:DateTime<UTC>,
    /// --inherited-- 
    pub updatedby:Option<Uuid>,
    /// has one
    pub wishlist:Option<Wishlist>,
}
