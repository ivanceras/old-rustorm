
extern crate rustorm;
#[macro_use]
extern crate rustorm_derive;
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


#[derive(IsDao)]
#[derive(IsTable)]
#[derive(Debug)]
struct Users{
    username: String,
    email: String,
}

fn test_delete() {
    let user = Users{
        username : "ivanceras".to_string(),
        email: "ivanceras@gmail.com".to_string()
    };
    let db = pool::db_with_url("postgres://postgres:p0stgr3s@localhost/mock").unwrap();
    let em = EntityManager::new(&*db);
    let filter = Filter::new("email", Equality::EQ, &"ivanceras@gmail.com".to_string());
    em.insert::<Users,Users>(&user);
    let ret:usize = em.delete::<Users>(&filter).unwrap();
    assert_eq!(ret, 1);
    println!("deleted : {} record(s)", ret);
}
