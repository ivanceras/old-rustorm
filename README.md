# Rustorm

[![Latest Version](https://img.shields.io/crates/v/rustorm.svg)](https://crates.io/crates/rustorm)
[![Build Status](https://api.travis-ci.org/ivanceras/rustorm.svg)](https://travis-ci.org/ivanceras/rustorm)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

An ORM for rust

## Features
* Fluent high-level and low-level API
* Composable queries
* Extensible to multiple database platform
* Easy to reason out generated SQL
* Optional model code generator

## [Documentation](http://ivanceras.github.io/rustorm/rustorm)



## Example


* Get all contents of product table


```rust

extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use uuid::Uuid;
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use rustc_serialize::json;

use rustorm::query::Query;
use rustorm::query::{Filter,Equality};
use rustorm::dao::{Dao,IsDao};
use rustorm::pool::ManagedPool;


#[derive(Debug, Clone)]
pub struct Product {
    pub product_id:Uuid,
    pub name:Option<String>,
    pub description:Option<String>,
}

impl IsDao for Product{
    fn from_dao(dao:&Dao)->Self{
        Product{
            product_id: dao.get("product_id"),
            name: dao.get_opt("name"),
            description: dao.get_opt("description"),
        }
    }
}


fn main(){
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    let mut pool = ManagedPool::init(&url, 1);
    let db = pool.connect().unwrap();
    
    let products: Vec<Product> = Query::select_all()
            .from_table("bazaar.product")
            .collect(db.as_ref());
    
    for prod in products{
        let name = prod.name.unwrap();
        let desc = match prod.description{
                        Some(desc) => desc,
                        None => "".to_string()
                    };
        println!("{}  {}  {:?}", prod.product_id, name, desc);
    }
}
```


* Get one photo of a product

```rust


#[derive(Debug, Clone)]
pub struct Photo {
    pub photo_id:Uuid,
    pub url:Option<String>,
}

impl IsDao for Photo{
    fn from_dao(dao:&Dao)->Self{
        Photo{
            photo_id: dao.get("photo_id"),
            url: dao.get_opt("url"),
        }
    }
}

fn main(){
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    let mut pool = ManagedPool::init(url, 1);
    let db = pool.connect().unwrap();
    
    let photo: Photo = Query::select_all()
                        .column("photo.url")
                        .from_table("bazaar.product")
                        .left_join_table("bazaar.product_photo",
                            "product.product_id", "product_photo.product_id")
                        .left_join_table("bazaar.photo",
                            "product_photo.photo_id", "photo.photo_id")
                        .filter("product.name", Equality::EQ, &"GTX660 Ti videocard")
                        .collect_one(db.as_ref());
                        
    println!("photo: {} {}",photo.photo_id, photo.url.unwrap());
}

```

* One complex query

```rust

fn main(){
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    let mut pool = ManagedPool::init(&url, 1);
    let db = pool.connect().unwrap();
    
    let mut query = Query::select_all();
    
    query.from_table("bazaar.product")
        .left_join_table("bazaar.product_category",
            "product_category.product_id", "product.product_id")
         .left_join_table("bazaar.category",
            "category.category_id", "product_category.category_id")
        .left_join_table("product_photo",
            "product.product_id", "product_photo.product_id")
        .left_join_table("bazaar.photo", 
            "product_photo.photo_id", "photo.photo_id")
        .filter("product.name", Equality::EQ, &"GTX660 Ti videocard")
        .filter("category.name", Equality::EQ, &"Electronic")
        .group_by(vec!["category.name"])
        .having("count(*)", Equality::GT, &1)
        .asc("product.name")
        .desc("product.created")
        ;
    let frag = query.build(db.as_ref());
    
    let expected = "
SELECT *
 FROM bazaar.product
    LEFT OUTER JOIN bazaar.product_category 
        ON product_category.product_id = product.product_id 
    LEFT OUTER JOIN bazaar.category 
        ON category.category_id = product_category.category_id 
    LEFT OUTER JOIN product_photo 
        ON product.product_id = product_photo.product_id 
    LEFT OUTER JOIN bazaar.photo 
        ON product_photo.photo_id = photo.photo_id 
    WHERE product.name = $1 
        AND category.name = $2 
    GROUP BY category.name 
    HAVING count(*) > $3 
    ORDER BY product.name ASC, product.created DESC".to_string();
    println!("actual:   {{{}}} [{}]", frag.sql, frag.sql.len());
    println!("expected: {{{}}} [{}]", expected, expected.len());
    assert!(frag.sql.trim() == expected.trim());
    
}
```

## Supported Database
 - [x] PostgreSQL
 - [x] Sqlite
 - [ ] MySQL
 - [ ] Oracle
 

## Roadmap

* Support for Oracle
* Support for MySql


# For Updates
Follow me on twitter: [@ivanceras](https://twitter.com/ivanceras)

# Support this project
* This is a 1 man show, so if you feel generous, please support this project at bountysource
[bountysource](https://www.bountysource.com/teams/rustorm)

* Suggestions are much welcome!
