extern crate rustc_serialize;
extern crate uuid;
extern crate chrono;

use uuid::Uuid;
use std::collections::BTreeMap;
use rustc_serialize::json::{self, Json, ToJson};
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use std::collections::HashMap;

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct TestStruct {
    data_int: u8,
    data_str: String,
    data_vector: Vec<u8>,
    id:Uuid,
    comment:Option<String>,
    created:DateTime<UTC>,
    updated:Option<DateTime<UTC>>,
}

fn main() {
    let input_data = TestStruct {
        data_int: 1,
        data_str: "madoka".to_string(),
        data_vector: vec![2,3,4,5],
        id:Uuid::new_v4(),
        comment:Some("wolla a hella".to_string()),
        created:UTC::now(),
        updated:Some(UTC::now()),
    };
    

    // Deserialize like before
    let json_str:String = json::encode(&input_data).unwrap();
    println!("\njson: {}", json_str);
    
    let mut map= HashMap::new();
    map.insert("data_int".to_string(), 1u32.to_json());
    map.insert("data_str".to_string(), "madoka".to_string().to_json());
    map.insert("data_vector".to_string(), vec![2.to_json(), 3.to_json()].to_json());
    
    println!("\nmap: {:?}", map);
    let json_obj:Json = map.to_json();
    println!("json_obj: {}", json_obj);
    println!("is json_object: {}", json_obj.is_object());
    println!("\t unwrap: {:?}", json_obj.as_object().unwrap());
    
    match json_obj{
        Json::Object(btree) => {
            println!("object..");
            println!("\tdata_int {:?}", btree.get("data_int"));
        },
        _ => println!("non object..")
    };
}

