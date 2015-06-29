extern crate rustorm;

use rustorm::database::DbConfig;

fn main(){
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    let config = DbConfig::from_url(url);
}