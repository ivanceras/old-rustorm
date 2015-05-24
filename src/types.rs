use std::collections::HashMap;
use uuid::Uuid;
use chrono::datetime::DateTime;
use chrono::naive::date::NaiveDate;
use chrono::naive::time::NaiveTime;
use chrono::naive::datetime::NaiveDateTime;
use chrono::offset::utc::UTC;


#[derive(Debug)]
pub enum Types{
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    U32(u32),
    I64(i64),
    F32(f32),
    F64(f64),
    String(String),
    VecU8(Vec<u8>),
    Map(HashMap<String, Option<String>>),
    Uuid(Uuid),
    DateTime(DateTime<UTC>),
    NaiveDate(NaiveDate),
    NaiveTime(NaiveTime),
    NaiveDateTime(NaiveDateTime),
}

pub trait ToDbType{
    fn to_db_type(&self)->Types;
}

impl ToDbType for i8{
    fn to_db_type(&self)->Types{
        Types::I8(self.clone())
    }
}

impl <'a>ToDbType for &'a str{
    fn to_db_type(&self)->Types{
        Types::String(self.to_string())
    }
}

pub struct Dao<'a>{
    pub values:HashMap<&'a str, Types>
}

impl <'a>Dao<'a>{

    pub fn new()->Self{
        Dao{values:HashMap::new()}
    }
    
    pub fn set_value(&mut self, column: &'a str, value:&'a ToDbType){
        self.values.insert(column, value.to_db_type());
    }
    
    pub fn get_value(&self, column: &'a str)->Option<&Types>{
        self.values.get(column)
    }

}