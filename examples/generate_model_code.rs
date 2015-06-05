extern crate rustorm;

use rustorm::db::postgres::Postgres;
use rustorm::codegen;
use rustorm::codegen::Config;

fn main(){
    let pg:Result<Postgres,&str> = Postgres::new("postgres://postgres:p0stgr3s@localhost/bazaar_v6");
    match pg{
        Ok(pg) => {
            let config =  Config{
                    base_module:Some("gen".to_string()),
                    include_table_references:true,
                    use_condensed_name:true,
                    generate_table_meta:true,
                    meta_file:Some("meta".to_string()),
                    base_dir:"./examples".to_string(),
                };
            codegen::generate_all(&pg, &config);
        }
        Err(error) =>{
            println!("{}",error);
        }
    }
}

