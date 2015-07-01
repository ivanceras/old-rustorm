# Rustorm

[![Latest Version](https://img.shields.io/crates/v/rustorm.svg)](https://crates.io/crates/rustorm)
[![Build Status](https://api.travis-ci.org/ivanceras/rustorm.svg)](https://travis-ci.org/ivanceras/rustorm)

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

fn main(){
    let pg= Postgres::with_connection("postgres://postgres:p0stgr3s@localhost/bazaar_v6");
    let products:Vec<Product> = Query::select()
                .enumerate_table_all_columns(&Product::table())
                .from::<Product>()
                .collect(&pg);

    for prod in products{
        println!("{}  {}  {:?}", prod.product_id, prod.name.unwrap(), prod.description);
    }
}

```


* Get one photo of a product

```rust


fn main(){
    let pg= Postgres::with_connection("postgres://postgres:p0stgr3s@localhost/bazaar_v6");
     let photo: Photo = Query::select()
                        .from::<Product>()
                        .left_join(&ProductPhoto::table(),
                            product::product_id, product_photo::product_id)
                        .left_join(&Photo::table(),
                            product_photo::photo_id, photo::photo_id)
                        .filter(product::name, Equality::EQ, &"GTX660 Ti videocard")
                        .collect_one(&pg);
    println!("debug: {:?}", photo);
}
```

* Get one exact match of a product

```rust
fn main(){
    let pg= Postgres::with_connection("postgres://postgres:p0stgr3s@localhost/bazaar_v6");
    let em = EntityManager::new(&pg);
    let pid = Uuid::parse_str("6db712e6-cc50-4c3a-8269-451c98ace5ad").unwrap();
    let prod: Product = em.get_exact(&pid);
    println!("{}  {}  {:?}", prod.product_id, prod.name.unwrap(), prod.description);
}
```

* One complex query

```rust

    let pg = Postgres::new();
    let mut query = Query::select();
    query.from::<Product>()
        .enumerate_table_all_columns(&Photo::table())
        .left_join(&ProductCategory::table(),
            product_category::product_id, product::product_id)
         .left_join(&Category::table(),
            category::category_id, product_category::category_id)
        .left_join(&ProductPhoto::table(),
            product::product_id, product_photo::product_id)
        .left_join(&Photo::table(), 
            product_photo::photo_id, photo::photo_id)
        .filter(product::name, Equality::EQ, &"GTX660 Ti videocard")
        .filter(category::name, Equality::EQ, &"Electronic")
        .group_by(vec![category::name])
        .having("count(*)", Equality::GT, &1)
        .asc(product::name)
        .desc(product::created)
        ;
    let frag = query.build(&pg);
    
    let expected = "
SELECT photo.organization_id, photo.client_id, photo.created, photo.created_by, 
    photo.updated, photo.updated_by, photo.priority, photo.name, photo.description, 
    photo.help, photo.active, photo.photo_id, photo.url, photo.data, 
    photo.seq_no
 FROM bazaar.product
    LEFT OUTER JOIN bazaar.product_category 
        ON product_category.product_id = product.product_id 
    LEFT OUTER JOIN bazaar.category 
        ON category.category_id = product_category.category_id 
    LEFT OUTER JOIN bazaar.product_photo 
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

```
## Todo list
* implement all DML API
* start working on DDL API
* start support in Sqlite

## Roadmap

* Support for Sqlite
* Support for Oracle
* Support for MySql


# For Updates
Follow me on twitter: [@ivanceras](https://twitter.com/ivanceras)

# Support this project
* This is a 1 man show, so if you feel generous, please support this project at bountysource
[bountysource](https://www.bountysource.com/teams/rustorm)

* Suggestions are much welcome!
