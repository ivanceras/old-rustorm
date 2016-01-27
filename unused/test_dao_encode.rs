extern crate rustorm;
extern crate chrono;
extern crate uuid;
extern crate rustc_serialize;

use rustorm::dao::Dao;
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use rustc_serialize::json;

fn main() {
    let s = "lee";
    let n = 20i8;
    let date = UTC::now();
    let mut dao = Dao::new();
    dao.set("name", &s);
    dao.set("age", &n);
    dao.set("created", &date);
    let _: String = dao.get("name");
    let _: i8 = dao.get("age");
    let _: DateTime<UTC> = dao.get("created");
    let _: Option<u8> = dao.get_opt("none");
    println!("json: {}", json::encode(&dao).unwrap());
}
