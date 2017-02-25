
extern crate rustorm;

use rustorm::query::Query;
use rustorm::query::Filter;
use rustorm::query::Select;
use rustorm::query::Equality::EQ;
use rustorm::database::BuildMode;
use rustorm::platform::pool::Platform;
use rustorm::platform::Postgres;
use rustorm::platform::pool;
use rustorm::query::Insert;


fn main(){
    println!("connecting....");
    let db = pool::db_with_url("postgres://postgres:p0stgr3s@localhost/mock").unwrap();
    println!("got connection");
    let mut query = Insert::into(&"users");
    query.columns(vec!["username", "email"]);
    query.values(vec![&"Lee".to_string(), 
        &"ivanceras@gmail.com".to_string()]);
    query.return_columns(vec!["*"]);
    let sql = query.debug_build(db.as_ref());
    println!("sql: {}", sql);
    let ret = db.as_ref().insert(&query);
    println!("{:#?}", ret);
}
