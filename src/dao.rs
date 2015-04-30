use std::collections::HashMap;
use postgres::ToSql;

pub struct DAO<'a>{
	entries:HashMap<String, &'a ToSql>,
}

impl <'a>DAO<'a>{
	
	pub fn new()->Self{
		DAO{entries:HashMap::new()}
	}
	
	fn set(&mut self, column:String, value: &'a ToSql){
		self.entries.insert(column, value);
	}
	
	fn get(&self, column: String)->Option<&'a ToSql>{
		let val = self.entries.get(&column);
		let val = val.unwrap();
		Some(*val)
	}
}