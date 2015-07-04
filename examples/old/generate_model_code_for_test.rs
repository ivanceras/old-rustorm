extern crate rustorm;

use rustorm::db::postgres::Postgres;
use rustorm::codegen;
use rustorm::codegen::Config;

/// this will generate needed model code for tests in ./tests/gen directory
fn main(){
    let pg = Postgres::connect_with_url("postgres://postgres:p0stgr3s@localhost/bazaar_v6").unwrap();
    let config =  Config{
            base_module:Some("gen".to_string()),
            include_table_references:true,
            use_condensed_name:true,
            generate_table_meta:true,
            base_dir:"./tests".to_string(),
        };
    codegen::generate_all(&pg, &config);
}

