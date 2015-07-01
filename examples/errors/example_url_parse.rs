extern crate url;
extern crate rustorm;
use rustorm::database::DbConfig;

use url::{Url, SchemeData};
use url::SchemeData::Relative;
use url::Host::Domain;
use url::RelativeSchemeData;


fn main(){
    let parsed = Url::parse(
        "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open"
    ).unwrap();
    println!("correct: {:?}", parsed);
    
    let parsed = Url::parse(
        "postgres://postgres:p0stgr3s@localhost/bazaar_v6"
    ).unwrap();
    println!("postgres: {:?}", parsed);
    
     let parsed = Url::parse(
        "https://postgres:p0stgr3s@localhost/bazaar_v6"
    ).unwrap();
     assert!( parsed == Url { 
             scheme: "https".to_string(), 
             query: None,
             fragment: None,
             scheme_data: Relative(
                 RelativeSchemeData { 
                     username: "postgres".to_string(), 
                     password: Some("p0stgr3s".to_string()), 
                     host: Domain("localhost".to_string()), 
                     port: None, 
                     default_port: Some(443), 
                     path: vec!["bazaar_v6".to_string()] 
                 })
             });
                     
    println!("hacky parse: {:?}", parsed);
    
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    let config = DbConfig::from_url(url);
    println!("config: {:?}", config);
    println!("url: {}",config.get_url());
}