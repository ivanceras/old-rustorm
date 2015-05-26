extern crate rustorm;

use rustorm::db::postgres::Postgres;
use rustorm::codegen;

fn main(){
    let pg:Result<Postgres,&str> = Postgres::new("postgres://postgres:p0stgr3s@localhost/bazaar_v5");
    match pg{
        Ok(pg) => {
            codegen::generate_all(&pg,"./gen/structs.rs", "./gen/is_table.rs");
        }
        Err(error) =>{
            println!("{}",error);
        }
    }
}

