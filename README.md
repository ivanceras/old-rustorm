# rustorm

[![Latest Version](https://img.shields.io/crates/v/rustorm.svg)](https://crates.io/crates/rustorm)

A Work in Progress rust ORM library for Postgresql

A fork of my previous, very simple [ORM](http://github.com/ivanceras/orm) which was written in java


## Dependency
* rust-postgres

## Features

* intelligent model code generation (The only functional part for now)
   * Can figure out linker tables, then build 1:M relation with the tables on the generated code
   * Can figure out extension tables, which is just 1:1 relation with another table

### To see it in action:

Put the sample database in your local postgresql installation

```bash
psql -U postgres -h localhost -W -d <bazaar_v6> -f ./scripts/bazaar_v6_all.sql

```
### Alternatively, you cam

```bash
sh setup.sh

```
Look at the code `./examples/generate_model_code.rs`
This will be the code to generate the model code based on the table schema and relationships to other tables.

```rust
///generate_model_code.rs

extern crate rustorm;

use rustorm::db::postgres::Postgres;
use rustorm::codegen;
use rustorm::codegen::Config;

fn main(){
    let pg:Result<Postgres,&str> = Postgres::new("postgres://postgres:p0stgr3s@localhost/bazaar_v6");
    match pg{
        Ok(pg) => {
            let config =  Config{
                    base_module:Some("gen".to_string()),
                    include_table_references:true,
                    use_condensed_name:true,
                    generate_table_meta:true,
                    base_dir:"./examples".to_string(),
                };
            codegen::generate_all(&pg, &config);
        }
        Err(error) =>{
            println!("{}",error);
        }
    }
}

```

```bash
cargo run --release --example generate_model_code
cat ./examples/gen/bazaar/product.rs
```



### Product table as an example
This is an example table `product` in schema `bazaar` `(./scripts/bazaar_v6_dump_schema.sql)`

```sql


CREATE TABLE bazaar.product
(
-- Inherited from table system.record:  organization_id uuid, -- @Value(users.user_id) , which means the value will be set with the users.user_id value...
-- Inherited from table system.record:  client_id uuid, -- @Value(users.client_id) The client_id of the user creating this records
-- Inherited from table system.record:  created timestamp with time zone NOT NULL DEFAULT now(),
-- Inherited from table system.record:  created_by uuid, -- @Value(users.user_id)
-- Inherited from table system.record:  updated timestamp with time zone NOT NULL DEFAULT now(),
-- Inherited from table system.record:  updated_by uuid, -- @Value(users.user_id)
-- Inherited from table system.record:  priority numeric,
-- Inherited from table system.record:  name character varying, -- This is @Required it has @DisplayLength(50) - 50 character in display length a @MinLength(1) and @MaxLength(100) - Do not go over 100 characters or else the system will throw a ValueTooLong exception...
-- Inherited from table system.record:  description character varying, -- @DisplayLength(100) When building a UI for this field...
-- Inherited from table system.record:  help text,
-- Inherited from table system.record:  active boolean NOT NULL DEFAULT true, -- @Active
  product_id uuid NOT NULL DEFAULT uuid_generate_v4(),
  parent_product_id uuid,
  is_service boolean DEFAULT false,
  price numeric,
  use_parent_price boolean DEFAULT false,
  unit character varying,
  tags json,
  info json, -- {color:"red",...
  seq_no integer, -- @Sequence can be used to do alternate ordering of the values, when alphetical or time can not be used
  upfront_fee numeric DEFAULT 0.00, -- Applicable to services, usually services has an upfront fee
  barcode character varying, -- barcode if scanning the product, conflict can happen, expect to return matching list of products using the barcode
  owner_id uuid, -- Whom this product belongs, since created_by can be someone else create the product list in behalf of the owner of the product
  currency_id uuid,
  CONSTRAINT product_pkey PRIMARY KEY (product_id),
  CONSTRAINT product_currency_id_fkey FOREIGN KEY (currency_id)
      REFERENCES payment.currency (currency_id) MATCH SIMPLE
      ON UPDATE CASCADE ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED,
  CONSTRAINT product_user_id_fkey FOREIGN KEY (owner_id)
      REFERENCES bazaar.users (user_id) MATCH SIMPLE
      ON UPDATE CASCADE ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED
)
INHERITS (system.record)

```

Look at the file `./examples/gen/bazaar/product.rs`
The generated rust code should look like

```rust
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
```

Take notice of these last members of the struct here

```rust

	  /// has one, extension table
    pub product_availability:Option<Box<ProductAvailability>>,
    /// has many, indirect referring table, derived from linker table: product_category
    pub category:Option<Vec<Category>>,
    /// has many, indirect referring table, derived from linker table: product_photo
    pub photo:Option<Vec<Photo>>,
    /// has many, indirect referring table, derived from linker table: product_review
    pub review:Option<Vec<Review>>,
```

* The code generator can figure out that product_availability is just an extension table of the product table.
* It also knows that product:categories has 1:M relationship hinted by the linker table product_category, and so with the Photos and Reviews.
* The `created` column has a data type `DateTime<UTC>`, since it is not nullable, while nullable columns such as `created_by` will be wrapped with `Option<Uuid>`,

## I have the generated model code, Now what?


```rust

use gen::bazaar::Product;

// this is the generated module, generated by generate_model_code example
mod gen;


fn main(){
    let pg:Result<Postgres,&str> = Postgres::new("postgres://postgres:p0stgr3s@localhost/bazaar_v6");
    match pg{
        Ok(pg) => {
            let sql = "select * from bazaar.product".to_string();
            let stmt = pg.conn.prepare(&sql).unwrap();
            for row in stmt.query(&[]).unwrap() {
                let product = Product{
                    product_id: row.get("product_id"),
                    barcode: row.get_opt("barcode").ok(),
                    currency_id: row.get_opt("currency_id").ok(),
                    info: row.get_opt("info").ok(),
                    is_service: row.get_opt("is_service").ok(),
                    owner_id: row.get_opt("owner_id").ok(),
                    parent_product_id: row.get_opt("parent_product_id").ok(),
                    price: row.get_opt("price").ok(),
                    seq_no: row.get_opt("seq_no").ok(),
                    tags: row.get_opt("tags").ok(),
                    unit: row.get_opt("unit").ok(),
                    upfront_fee: row.get_opt("upfront_fee").ok(),
                    use_parent_price: row.get_opt("use_parent_price").ok(),
                    active: row.get("active"),
                    client_id: row.get_opt("client_id").ok(),
                    created: row.get("created"),
                    created_by: row.get_opt("created_by").ok(),
                    description: row.get_opt("description").ok(),
                    help: row.get_opt("help").ok(),
                    name: row.get_opt("name").ok(),
                    organization_id: row.get_opt("organization_id").ok(),
                    priority:row.get_opt("priority").ok(),
                    updated:row.get("updated"),
                    updated_by: row.get_opt("updated_by").ok(),
                    owner:None,
                    currency:None,
                    product_availability:None,
                    category:None,
                    photo:None,
                    review:None
                };
                println!("product:{}({})", product.name.as_ref().unwrap(), product.product_id);
                println!("{},", json::as_pretty_json(&product));
            }
        }
        Err(error) =>{
            println!("{}",error);
        }
    }
}


```

```sh

cargo run --release --example test_product

```
```sh
product:iphone4s(f7521093734d488a9f60fc9f11f7e750)
...

```




##Documentation
[docs](http://ivanceras.github.io/rustorm/rustorm)

##Roadmap
* Implement all the features in ivanceras/orm
  * Automatic conversion of Row to objects (of the generated model)
  * Automatic conversion of HashMap to objects
  * Implement the declarative Query API
* become a full blown ORM for rust
* Support for sqlite



# For Updates
Follow me on twitter: [@ivanceras](https://twitter.com/ivanceras)
