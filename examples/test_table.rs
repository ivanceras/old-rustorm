extern crate rustorm;

use rustorm::table::Table;
use rustorm::table::Column;

pub fn main(){
	println!("testing here..");
	let c1 = Column{name:"product_id".to_string(),
			data_type:"Uuid".to_string(),
			is_primary:true,
			is_unique:false,
			default:None,
			comment:None,
			not_null:false,
			foreign:None,
			is_inherited:false,
		};
	let c2 = Column{name:"description".to_string(),
			data_type:"String".to_string(),
			is_primary:true,
			is_unique:false,
			default:None,
			comment:None,
			not_null:false,
			foreign:None,
			is_inherited:false,
		};
	
		let c3 = Column{name:"description".to_string(),
			data_type:"String33".to_string(),
			is_primary:true,
			is_unique:false,
			default:None,
			comment:None,
			not_null:false,
			foreign:None,
			is_inherited:false,
		};
	let v1 = vec![&c1, &c2];
	let v2 = vec![&c1, &c3];
	assert!(v1 == v2, "Not equal");
}
