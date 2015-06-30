extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;
extern crate bazaar;


use rustorm::db::postgres::Postgres;
use rustorm::codegen;
use uuid::Uuid;
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use rustc_serialize::json;

use rustorm::em::EntityManager;
use rustorm::table::IsTable;
use rustorm::dao::IsDao;
use rustorm::query::Query;
use rustorm::dao::Type;
use rustorm::query::{Filter,Equality,Operand};
use bazaar::gen::bazaar::Category;
use bazaar::gen::bazaar::category;
use rustorm::dao::Dao;
use bazaar::gen::bazaar::Product;
use bazaar::gen::bazaar::product;
use bazaar::gen::bazaar::ProductAvailability;
use bazaar::gen::bazaar::product_availability;

 
#[test]
fn test_select_filter(){
    let pg:Result<Postgres,&str> = Postgres::new("postgres://postgres:p0stgr3s@localhost/bazaar_v6");
       match pg{
        Ok(pg) => {
            let em = EntityManager::new(&pg);
            let mut query = Query::new();
            query.from_table(&Product::table());
            query.enumerate_table_all_columns(&Product::table());
            
            query.left_join(&ProductAvailability::table(), 
                product::product_id, product_availability::product_id);
            query.filter(product::name, Equality::LIKE, &"iphone%");
            
            query.add_filter(
                Filter::new(product::description, Equality::LIKE, 
                    Operand::Value(Type::String("%Iphone%".to_string())))
                );
            
            query.desc(product::created);
            query.asc(product::product_id);
            
            let sql = query.build(&pg);
            let expected = "SELECT product.organization_id, product.client_id, product.created, product.created_by, 
    product.updated, product.updated_by, product.priority, product.name, product.description, 
    product.help, product.active, product.product_id, product.parent_product_id, product.is_service, 
    product.price, product.use_parent_price, product.unit, product.tags, product.info, 
    product.seq_no, product.upfront_fee, product.barcode, product.owner_id, product.currency_id
 FROM bazaar.product
    LEFT OUTER JOIN bazaar.product_availability 
        ON product.product_id = product_availability.product_id 
    WHERE product.name LIKE $1 
        AND product.description LIKE $2 
    ORDER BY product.created DESC, product.product_id ASC".to_string();
            
            println!("actual:   {} [{}]", sql.sql, sql.sql.len());
            println!("expected: {} [{}]", expected, expected.len());
            
            assert!(sql.sql == expected);
            assert!(sql.params.len() == 2);
            match sql.params[0]{
                Type::String(ref x) => assert!(x == "iphone%"),
                _ => (),
             };
            match sql.params[1]{
                Type::String(ref x) => assert!(x == "%Iphone%"),
                _ => (),
            };  
        }
        Err(error) =>{
            println!("{}",error);
        }
    }
}


#[test]
fn test_query_insert(){
    let pg:Result<Postgres,&str> = Postgres::new("postgres://postgres:p0stgr3s@localhost/bazaar_v6");
       match pg{
        Ok(pg) => {
            let em = EntityManager::new(&pg);
            let mut dao = Dao::new();
            dao.set("name", &"inserting 1 records");
            dao.set("description", &"testing insert 1 record to product");
            //let dao = em.insert(&Product::table(), dao);
            //let prod = Product::from_dao(&dao);
            //println!("created: {}", prod.created);
        }
        Err(error) =>{
            println!("{}",error);
        }
    }
}

#[test]
fn test_update_query(){
    let pg:Result<Postgres,&str> = Postgres::new("postgres://postgres:p0stgr3s@localhost/bazaar_v6");
       match pg{
        Ok(pg) => {
            let em = EntityManager::new(&pg);
            let mut query = Query::update();
            query.from::<Product>();
            query.enumerate_column(product::name);
            query.enumerate_all_table_column_as_return(&Product::table());
            query.value(&"iphone");
            query.filter(product::name, Equality::LIKE, &"aphone");
            
            query.add_filter(
                Filter::new(product::description, Equality::LIKE, 
                    Operand::Value(Type::String("%Iphone%".to_string())))
                );
            let sql = query.build(&pg);
            let expected = "UPDATE bazaar.product
SET name = $1 
    WHERE product.name LIKE $2 
        AND product.description LIKE $3 RETURNING organization_id, client_id, created, created_by, updated, updated_by, priority, name, description, help, active, product_id, parent_product_id, is_service, price, use_parent_price, unit, tags, info, seq_no, upfront_fee, barcode, owner_id, currency_id".to_string();
            println!("actual:   {} [{}]", sql.sql, sql.sql.len());
            println!("expected: {} [{}]", expected, expected.len());
            assert!(sql.sql == expected);
            
            assert!(sql.params.len() == 3);
            match sql.params[0]{
                Type::String(ref x) => assert!(x == "iphone"),
                _ => (),
             };
            match sql.params[1]{
                Type::String(ref x) => assert!(x == "aphone"),
                _ => (),
             };
            match sql.params[2]{
                Type::String(ref x) => assert!(x == "%Iphone%"),
                _ => (),
             };
        }
        Err(error) =>{
            println!("{}",error);
        }
    }
}


#[test]
fn test_query_delete_category(){
    let pg:Result<Postgres,&str> = Postgres::new("postgres://postgres:p0stgr3s@localhost/bazaar_v6");
       match pg{
        Ok(pg) => {
            let mut query = Query::delete();
            query.from::<Category>();
            query.filter(category::name, Equality::LIKE, &"test%");
            let sql = query.build(&pg);
            let expected = "DELETE FROM bazaar.category
    WHERE category.name LIKE $1 ".to_string();
            println!("actual:   {} [{}]", sql.sql, sql.sql.len());
            println!("expected: {} [{}]", expected, expected.len());
            assert!(sql.sql == expected);
            
            assert!(sql.params.len() == 1);
            match sql.params[0]{
                Type::String(ref x) => assert!(x == "test%"),
                _ => (),
             };
        }
        Err(error) =>{
            println!("{}",error);
        }
    }
}
