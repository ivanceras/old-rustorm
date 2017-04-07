extern crate rustorm;

#[cfg(test)] #[macro_use] extern crate pretty_assertions;


use rustorm::query::Query;
use rustorm::query::Filter;
use rustorm::query::Select;
use rustorm::query::Equality::EQ;
use rustorm::database::BuildMode;
use rustorm::platform::pool::Platform;
use rustorm::platform::Postgres;

#[cfg(test)]
#[test]
#[cfg(feature = "postgres")]
fn test_pg(){
    let pg = Platform::pg();
    let mut query = Select::new();
    query.columns(vec!["username", "email"]);
    query.from(&"users".to_string());
    let filter = Filter::new("username", EQ, &"Hello".to_string());
    query.add_filter(&filter);   
    let sql = pg.build_select(&query, &BuildMode::Debug); 
    println!("{}", sql);
    let expected = r#"
   SELECT username, email
     FROM users
    WHERE username = 'Hello'
    "#;
    assert_eq!(sql.sql.trim(), expected.trim());
}

#[test]
#[cfg(feature = "sqlite")]
fn test_sqlite(){
    let pg = Platform::sqlite();
    let mut query = Select::new();
    query.from(&"users".to_string());
    let filter = Filter::new("username", EQ, &"Hello".to_string());
    query.add_filter(&filter);   
    let sql = pg.build_select(&query, &BuildMode::Debug); 
    println!("{}", sql);
    let expected = r#"
   SELECT 
     FROM users
    WHERE username = 'Hello'
    "#;
    assert_eq!(sql.sql.trim(), expected.trim());
}


#[test]
#[cfg(feature = "mysql")]
fn test_mysql(){
    let pg = Platform::mysql();
    let mut query = Select::new();
    query.from(&"users".to_string());
    let filter = Filter::new("username", EQ, &"Hello".to_string());
    query.add_filter(&filter);   
    let sql = pg.build_select(&query, &BuildMode::Debug); 
    println!("{}", sql);
    let expected = r#"
   SELECT 
     FROM users
    WHERE username = 'Hello'
    "#;
    assert_eq!(sql.sql.trim(), expected.trim());
}

