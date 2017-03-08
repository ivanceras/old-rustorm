extern crate rustorm;
#[macro_use]
extern crate rustorm_derive;
extern crate uuid;
extern crate chrono;
use rustorm::dao::Dao;
use rustorm::dao::IsDao;
use rustorm::query::TableName;
use rustorm::query::IsTable;
use rustorm::query::ColumnName;
use rustorm::dao::ToValue;
use rustorm::dao::FromValue;
use rustorm::platform::pool;
use rustorm::entity::EntityManager;
use rustorm::query::Filter;
use rustorm::query::Equality;
use uuid::Uuid;
use chrono::DateTime;
use chrono::UTC;




#[derive(IsDao)]
#[derive(IsTable)]
#[derive(Debug)]
struct Users{
    user_id: Uuid,
    username: String,
    email: String,
    created: DateTime<UTC>,
    updated: DateTime<UTC>,
    active: bool
}

fn main() {
    let db = pool::db_with_url("sqlite:///file.db").unwrap();
    let em = EntityManager::new(&*db);
    let filter = Filter::new("email", Equality::EQ, &"ivanceras@gmail.com".to_string());
    let ret:Vec<Users> = em.get_all().unwrap();
    println!("got : {:#?}", ret);
    let filtered:Vec<Users> = em.get_all_with_filter(&filter).unwrap();
    println!("filtered: {:#?}", filtered);
}
