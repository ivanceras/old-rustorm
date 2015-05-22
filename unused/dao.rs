use std::collections::HashMap;
use std::fmt::{Display, Formatter,Result};

pub trait SqlType{

}

impl Display for SqlType{
	 fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "sql type")
    }
}

impl SqlType for String{

}


pub struct SDao<'a> {
    values:HashMap<&'a str, &'a SqlType>
}

impl<'a> SDao<'a> {
	
	pub fn new()->Self{
		SDao { values:HashMap::new()}
	}
	
    pub fn get_value(&self, column:&str)->Option<&&SqlType>{
        self.values.get(column)
    }
    
    pub fn set_value(&mut self, column:&'a str, value:&'a SqlType){
		self.values.insert(column, value);
	}
}

pub struct Dao<'a> {
    values:HashMap<&'a str, &'a str>
}

impl<'a> Dao<'a> {
	
	pub fn new()->Self{
		Dao { values:HashMap::new()}
	}
	
    pub fn get_value(&self, column:&str)->Option<&&str>{
        self.values.get(column)
    }
    
    pub fn set_value(&mut self, column:&'a str, value:&'a str){
		self.values.insert(column, value);
	}
}