extern crate rustorm;

use rustorm::db::postgres::Postgres;
use rustorm::codegen;
use rustorm::codegen::Config;

fn main(){
    let pg:Result<Postgres,&str> = Postgres::new("postgres://postgres:p0stgr3s@localhost/bazaar_v6");
    match pg{
        Ok(pg) => {
            let config = Config::default();
            codegen::generate_all(&pg, &config);
        }
        Err(error) =>{
            println!("{}",error);
        }
    }
}

