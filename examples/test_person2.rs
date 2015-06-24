extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;
extern crate postgres;



use rustorm::db::postgres::Postgres;
use rustorm::codegen;
use uuid::Uuid;
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use rustc_serialize::json;
use postgres::types::ToSql;


struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}


fn main(){
    let pg:Result<Postgres,&str> = Postgres::new("postgres://postgres:p0stgr3s@localhost/test");
    match pg{
        Ok(pg) => {
            let conn = pg.conn;
            let sql = "INSERT INTO person (name, data) VALUES ($1, $2)".to_string();
            let me = Person {
                    id: 0,
                    name: "Steven".to_string(),
                    data: None
                };
            let mut param:Vec<&ToSql> = vec![&me.name];
            param.push(&me.data);
            
            conn.execute(&sql, &param).unwrap();
            
            let stmt = conn.prepare("SELECT id, name, data FROM person").unwrap();
            for row in stmt.query(&[]).unwrap() {
                let person = Person {
                    id: row.get(0),
                    name: row.get(1),
                    data: row.get(2)
                };
                println!("Found person {}", person.name);
            }
        }
        Err(error) =>{
            println!("{}",error);
        }
    }
}
