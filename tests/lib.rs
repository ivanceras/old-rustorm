extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;


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
use rustorm::dao::Dao;
use gen::bazaar::Product;
use gen::bazaar::product;
use gen::bazaar::ProductAvailability;
use gen::bazaar::product_availability;
use gen::bazaar::ProductCategory;
use gen::bazaar::product_category;
use gen::bazaar::Category;
use gen::bazaar::category;
use gen::bazaar::ProductPhoto;
use gen::bazaar::product_photo;
use gen::bazaar::Photo;
use gen::bazaar::photo;

use rustorm::database::Pool;


mod gen;

mod test_pool;
 
#[test]
fn test_select_filter(){
    let pg = Postgres::new();
    let em = EntityManager::new(&pg);
    let mut query = Query::new();
    query.from(&Product::table());
    query.enumerate_table_all_columns(&Product::table());
    
    query.left_join(&ProductAvailability::table(), 
        product::product_id, product_availability::product_id);
    query.filter(product::name, Equality::LIKE, &"iphone%");
    
    query.add_filter(
        Filter::new(product::description, Equality::LIKE, &"%Iphone%")
        );
    
    query.desc(product::created);
    query.asc(product::product_id);
    
    let frag = query.build(&pg);
    let expected = "
SELECT product.organization_id, product.client_id, product.created, product.created_by, 
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
    
    println!("actual:   {} [{}]", frag.sql, frag.sql.len());
    println!("expected: {} [{}]", expected, expected.len());
    
    assert!(frag.sql.trim() == expected.trim());
    assert!(frag.params.len() == 2);
    match frag.params[0]{
        Type::String(ref x) => assert!(x == "iphone%"),
        _ => (),
     };
    match frag.params[1]{
        Type::String(ref x) => assert!(x == "%Iphone%"),
        _ => (),
    };  
}

#[test]
fn test_update_query(){
    let pg = Postgres::new();
    let em = EntityManager::new(&pg);
    let mut query = Query::update();
    query.from(&Product::table());
    query.enumerate_column(product::name);
    query.enumerate_all_table_column_as_return(&Product::table());
    query.value(&"iphone");
    query.filter(product::name, Equality::LIKE, &"aphone");
    
    query.add_filter(Filter::new(product::description, Equality::LIKE, &"%Iphone%"));
    let frag = query.build(&pg);
    let expected = "
UPDATE bazaar.product
SET name = $1 
    WHERE name LIKE $2 
        AND description LIKE $3 RETURNING organization_id, client_id, created, created_by, updated, updated_by, priority, name, description, help, active, product_id, parent_product_id, is_service, price, use_parent_price, unit, tags, info, seq_no, upfront_fee, barcode, owner_id, currency_id ".to_string();
    println!("actual:   {} [{}]", frag.sql, frag.sql.len());
    println!("expected: {} [{}]", expected, expected.len());
    assert!(frag.sql.trim() == expected.trim());
    
    assert!(frag.params.len() == 3);
    match frag.params[0]{
        Type::String(ref x) => assert!(x == "iphone"),
        _ => (),
     };
    match frag.params[1]{
        Type::String(ref x) => assert!(x == "aphone"),
        _ => (),
     };
    match frag.params[2]{
        Type::String(ref x) => assert!(x == "%Iphone%"),
        _ => (),
     };
}


#[test]
fn test_query_delete_category(){
    let pg = Postgres::new();
    let mut query = Query::delete();
    query.from(&Category::table());
    query.filter(category::name, Equality::LIKE, &"test%");
    let frag = query.build(&pg);
    let expected = "DELETE FROM bazaar.category
    WHERE name LIKE $1 ".to_string();
    println!("actual:   {} [{}]", frag.sql, frag.sql.len());
    println!("expected: {} [{}]", expected, expected.len());
    assert!(frag.sql.trim() == expected.trim());
    
    assert!(frag.params.len() == 1);
    match frag.params[0]{
        Type::String(ref x) => assert!(x == "test%"),
        _ => (),
     };
}



#[test] 
fn test_join(){
    let pg = Postgres::new();
    let mut query = Query::new();
    query.from(&Product::table());
    query.enumerate_table_all_columns(&Product::table());
    
    query.left_join(&ProductAvailability::table(), 
        product::product_id, product_availability::product_id);
    
    let frag = query.build(&pg);
    
    let expected = "
SELECT product.organization_id, product.client_id, product.created, product.created_by, 
    product.updated, product.updated_by, product.priority, product.name, product.description, 
    product.help, product.active, product.product_id, product.parent_product_id, product.is_service, 
    product.price, product.use_parent_price, product.unit, product.tags, product.info, 
    product.seq_no, product.upfront_fee, product.barcode, product.owner_id, product.currency_id
 FROM bazaar.product
    LEFT OUTER JOIN bazaar.product_availability 
        ON product.product_id = product_availability.product_id ".to_string();
    println!("actual:   {} [{}]", frag.sql, frag.sql.len());
    println!("expected: {} [{}]", expected, expected.len());
    assert!(frag.sql.trim() == expected.trim());
}

#[test]
fn test_complex(){
    let pg = Postgres::new();
    let mut query = Query::select();
    let mut filters = Filter::new(product::name, Equality::EQ, &"GTX660 Ti videocard");
    filters.and(category::name, Equality::EQ, &"Electronic");
    query.all()
        .from(&Product::table())
        .left_join(&ProductCategory::table(),
           product_category::product_id, product::product_id)
        .left_join(&Category::table(),
            category::category_id, product_category::category_id)
        .left_join(&ProductPhoto::table(),
            product::product_id, product_photo::product_id)
        .left_join(&Photo::table(), 
            product_photo::photo_id, photo::photo_id)
        .add_filter(filters)
        .asc(product::name)
        .desc(product::created)
        .group_by(vec![category::name])
        .having("count(*)", Equality::GT, &1)
        ;
    let frag = query.build(&pg);
    
    let expected = "
SELECT *
 FROM bazaar.product
    LEFT OUTER JOIN bazaar.product_category 
        ON product_category.product_id = product.product_id 
    LEFT OUTER JOIN bazaar.category 
        ON category.category_id = product_category.category_id 
    LEFT OUTER JOIN bazaar.product_photo 
        ON product.product_id = product_photo.product_id 
    LEFT OUTER JOIN bazaar.photo 
        ON product_photo.photo_id = photo.photo_id 
    WHERE ( product.name = $1 AND category.name = $2  )
    GROUP BY category.name 
    HAVING count(*) > $3 
    ORDER BY product.name ASC, product.created DESC".to_string();
    println!("actual:   {{{}}} [{}]", frag.sql, frag.sql.len());
    println!("expected: {{{}}} [{}]", expected, expected.len());
    assert!(frag.sql.trim() == expected.trim());
}


#[test]
fn test_multiple_filters(){
    let pg = Postgres::new();
    let mut query = Query::select();
    query.from(&Product::table())
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
}


#[test]
fn test_complex_select_all(){
    let pg = Postgres::new();
    let mut query = Query::select();
    query.from(&Product::table())
        .all()
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
SELECT *
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
}



#[test]
fn test_flex_query(){
    let mut pool = Pool::init();
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    pool.reserve_connection(&url, 5);
    println!("{} connections..", pool.total_free_connections());
    let db = pool.get_db_with_url(&url).unwrap();
    
    let prod: Product = Query::select_all()
            .from_table("bazaar.product")
            .filter("name", Equality::EQ, &"GTX660 Ti videocard")
            .collect_one(db.as_ref());

    println!("{}  {}  {:?}", prod.product_id, prod.name.unwrap(), prod.description);
    pool.release(db);
}