extern crate rustorm;

use rustorm::query::Query;
use rustorm::query::Filter;
use rustorm::query::Select;
use rustorm::query::Equality::EQ;
use rustorm::database::BuildMode;
use rustorm::platform::pool::Platform;
use rustorm::platform::Postgres;
use rustorm::platform::pool;

fn main(){
    println!("connecting....");
    let db = pool::db_with_url("postgres://postgres:p0stgr3s@localhost/bazaar_v8").unwrap();
    println!("got connection");
    let mut query = Select::new();
    query.columns(vec!["username", "email"]);
    query.from(&"bazaar.users".to_string());
    query.set_limit(1);

    let ret = db.select(&query);
    println!("{:#?}", ret);
}
