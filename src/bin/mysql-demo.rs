extern crate rustorm;


use rustorm::platform::mysql::Mysql;
use rustorm::database::Database;
use rustorm::dao::Value;

pub fn main () {

    let mut conn = Mysql::connect_with_url("mysql://test:test@localhost/test").unwrap();
    println!("connected");
    let params: Vec<Value> = vec!();
    let mut cols: Vec<&str> = Vec::new();
    cols.push("v");
    let res = conn.execute_sql_with_return_columns("select version() as v", &params, cols);
    for row in &res{
        for (k,v) in &row.values {

            println!("{}: {}", k, v);
        }
    }
}
