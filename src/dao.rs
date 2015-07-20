use std::collections::HashMap;
use std::collections::BTreeMap;
use uuid::Uuid;
use chrono::datetime::DateTime;
use chrono::naive::date::NaiveDate;
use chrono::naive::time::NaiveTime;
use chrono::naive::datetime::NaiveDateTime;
use chrono::offset::utc::UTC;
use std::fmt;
use query::ColumnName;
use table::IsTable;
use rustc_serialize::{Decodable, Encodable,Encoder};
use rustc_serialize::json::{self, ToJson, Json};

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
///supported generic datatypes for an ORM
pub enum Value{
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
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
    Null,
}


/// custom implementation for value encoding to json,
/// does not include unnecessary enum variants fields.
impl Encodable for Value{
    
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        match *self {
            Value::Bool(ref x) => x.encode(s),
            Value::I8(ref x) => x.encode(s),
            Value::I16(ref x) => x.encode(s),
            Value::I32(ref x) => x.encode(s),
            Value::I64(ref x) => x.encode(s),
            Value::U8(ref x) => x.encode(s),
            Value::U16(ref x) => x.encode(s),
            Value::U32(ref x) => x.encode(s),
            Value::U64(ref x) => x.encode(s),
            Value::F32(ref x) => x.encode(s),
            Value::F64(ref x) => x.encode(s),
            Value::String(ref x) => x.encode(s),
            Value::VecU8(ref x) => x.encode(s),
            Value::Uuid(ref x) => x.encode(s),
            Value::DateTime(ref x) => x.encode(s),
            Value::NaiveDate(ref x) => x.encode(s),
            Value::NaiveTime(ref x) => x.encode(s),
            Value::NaiveDateTime(ref x) => x.encode(s),
            Value::Null => s.emit_nil(),
             _ => panic!("unsupported/unexpected type! {:?}", self),
        }
    }
}

impl ToJson for Value{
    
    fn to_json(&self) -> Json{
                match *self {
            Value::Bool(ref x) => x.to_json(),
            Value::I8(ref x) => x.to_json(),
            Value::I16(ref x) => x.to_json(),
            Value::I32(ref x) => x.to_json(),
            Value::I64(ref x) => x.to_json(),
            Value::U8(ref x) => x.to_json(),
            Value::U16(ref x) => x.to_json(),
            Value::U32(ref x) => x.to_json(),
            Value::U64(ref x) => x.to_json(),
            Value::F32(ref x) => x.to_json(),
            Value::F64(ref x) => x.to_json(),
            Value::String(ref x) => x.to_json(),
            Value::VecU8(ref x) => x.to_json(),
            Value::Uuid(ref x) => x.to_hyphenated_string().to_json(),
            Value::DateTime(ref x) => x.to_rfc3339().to_json(),
//            Value::NaiveDate(ref x) => x.to_json(),
//            Value::NaiveTime(ref x) => x.to_json(),
//            Value::NaiveDateTime(ref x) => x.to_json(),
            Value::Null => Json::Null,
             _ => panic!("unsupported/unexpected type! {:?}", self),
        }
    }
}


impl fmt::Display for Value{
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Bool(ref x) => write!(f, "'{}'", x),
            Value::I8(ref x) => write!(f, "'{}'", x),
            Value::I16(ref x) => write!(f, "'{}'", x),
            Value::I32(ref x) => write!(f, "'{}'", x),
            Value::I64(ref x) => write!(f, "'{}'", x),
            Value::U8(ref x) => write!(f, "'{}'", x),
            Value::U16(ref x) => write!(f, "'{}'", x),
            Value::U32(ref x) => write!(f, "'{}'", x),
            Value::U64(ref x) => write!(f, "'{}'", x),
            Value::String(ref x) => write!(f, "'{}'", x),
            Value::VecU8(ref x) => write!(f, "'{:?}'", x),
            Value::Uuid(ref x) => write!(f, "'{}'", x),
            Value::DateTime(ref x) => write!(f, "'{}'", x),
            Value::NaiveDate(ref x) => write!(f, "'{}'", x),
            Value::NaiveTime(ref x) => write!(f, "'{}'", x),
            Value::NaiveDateTime(ref x) => write!(f, "'{}'", x),
            Value::Null => write!(f, "'nil'"),
            _ => panic!("unsupported/unexpected type! {:?}", self),
        }
    }
}


/// trait for converting dao to model
/// sized and clonable
pub trait IsDao{
    
    /// convert dao to an instance of the corresponding struct of the model
    /// taking into considerating the renamed columns
    fn from_dao(dao: &Dao)->Self;
    
    /// convert from an instance of the struct to a dao representation
    /// to be saved into the database
    fn to_dao(&self)->Dao;
}

/// meta result of a query useful when doing complex query, and also with paging
#[derive(Debug)]
pub struct DaoResult{
    pub dao: Vec<Dao>,
    ///renamed columns for each table
    /// ie. product => [(name, product_name),..];
    pub renamed_columns: Vec< (ColumnName, String) >, 
    
    /// the total number of records
    pub total:Option<usize>,
    /// page of the query
    pub page: Option<usize>,
    /// page size
    pub page_size: Option<usize>,
}

impl DaoResult{
    /// get the list of renamed column name in matching table name
    fn get_renamed_columns(&self, table:&str)->Vec< (String, String) >{
        let mut columns = vec![];
        for &(ref col, ref rename) in &self.renamed_columns{
            if col.table.is_some(){
                if col.table.as_ref().unwrap() == table{
                    columns.push( (col.column.to_string(), rename.to_string()) );
                }
            }
        }
        columns
    }
    
    /// cast the dao to the specific struct instance
    /// do not include if non nullable parts contains null
    pub fn cast<T:IsTable+IsDao>(&self)->Vec<T>{
        let table = T::table();
        let non_nulls = table.non_nullable_columns();
        let mut obj = vec![];
        let renamed_columns = self.get_renamed_columns(&table.name);
        for dao in &self.dao{
            let mut dao_clone = dao.clone();
            dao_clone.correct_renamed_columns(&renamed_columns);
            if dao_clone.all_has_values(&non_nulls){
                let p = T::from_dao(&dao_clone);
                obj.push(p);
            }
        }
        obj
    }
    
    pub fn cast_one<T:IsTable+IsDao>(&self)->Option<T>{
        let mut casted = self.cast::<T>();
        if casted.len() < 1{
            return None;
        }
        assert!(casted.len() == 1);
        Some(casted.remove(0))
    }
}

#[derive(Debug, Clone)]
/// TODO: optimization, used enum types for the key values
/// This will save allocation of string to enum keys which is a few bytes, int 
pub struct Dao{
    //pub values:HashMap<String, Value>,
    pub values:BTreeMap<String, Value>,
}

/// custom Encoder for Dao,
/// decodes directly the content of `values`, instead of `values` as field of this `Dao` struct
impl Encodable for Dao{
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error>{
        self.values.encode(s)
    }
}

impl ToJson for Dao{
    
    fn to_json(&self) -> Json{
        let mut btree = BTreeMap::new();
        for (key, value) in &self.values{
            btree.insert(key.to_string(), value.to_json());
        }
        Json::Object(btree)
    }
    
}

impl Dao{

    pub fn new()->Self{
        //Dao{values:HashMap::new()}
        Dao{values:BTreeMap::new()}
    }
    
    pub fn set(&mut self, column: &str, value:&ToValue){
        self.values.insert(column.to_string(), value.to_db_type());
    }
    
    /// set to null the value of this column
    pub fn set_null(&mut self, column: &str){
        self.set_value(column, Value::Null);
    }
    
    pub fn set_value(&mut self, column: &str, value:Value){
        self.values.insert(column.to_string(), value);
    }
    pub fn get_value(&self, column: &str)->Value{
        let value = self.values.get(column);
        match value{
            Some(value) => value.clone(),
            None => panic!("No such value for {}", column),
        }
    }
    /// take the value and remove the content 
    pub fn remove<T>(&mut self, column: &str) -> T where T: FromValue{
        let value = self.values.remove(column).unwrap();
        FromValue::from_type(value)
    }

    /// take the value but not removing the content
    pub fn get<T>(&self, column: &str) -> T where T: FromValue{
        let value = self.values.get(column).unwrap();
        FromValue::from_type(value.clone())
    }
    /// get optional value
    pub fn get_opt<T>(&self, column: &str) -> Option<T> where T: FromValue{
        let value = self.values.get(column);
        if value.is_some(){
            let v = value.as_ref().unwrap().clone();
            match v{
                &Value::Null => None,
                _ => Some(FromValue::from_type(v.clone()))
            }
        }else{
            None
        }
    }
    
    /// get a reference of the type
    pub fn as_ref(&self, column: &str)->&Value{
        self.values.get(column).unwrap()
    }
    
    
    fn correct_renamed_columns(&mut self, renamed_columns:&Vec<(String, String)>){
        for &(ref column, ref rename) in renamed_columns{
            let value = self.get_value(rename);
            self.set_value(&column, value);
        }
    }
    
    fn all_has_values(&self, non_nulls: &Vec<String>)->bool{
        for column in non_nulls{
            let value = self.values.get(column);
            if value.is_none(){
                return false;
            }
            if value.is_some(){
                let v = value.as_ref().unwrap().clone();
                match v{
                    &Value::Null => return false,
                    _ => ()
                };
            }
        }
        true
    }
    
}


/// rename to ToValue
pub trait ToValue{
    fn to_db_type(&self)->Value;
}

impl ToValue for (){
    fn to_db_type(&self)->Value{
        Value::Null
    }
}

impl ToValue for bool{
    fn to_db_type(&self)->Value{
        Value::Bool(self.clone())
    }
}

/// signed INTs
impl ToValue for i8{
    fn to_db_type(&self)->Value{
        Value::I8(self.clone())
    }
}

impl ToValue for i16{
    fn to_db_type(&self)->Value{
        Value::I16(self.clone())
    }
}
impl ToValue for i32{
    fn to_db_type(&self)->Value{
        Value::I32(self.clone())
    }
}

impl ToValue for i64{
    fn to_db_type(&self)->Value{
        Value::I64(self.clone())
    }
}
/// unsigned INTs
impl ToValue for u8{
    fn to_db_type(&self)->Value{
        Value::U8(self.clone())
    }
}

impl ToValue for u16{
    fn to_db_type(&self)->Value{
        Value::U16(self.clone())
    }
}
impl ToValue for u32{
    fn to_db_type(&self)->Value{
        Value::U32(self.clone())
    }
}

impl ToValue for u64{
    fn to_db_type(&self)->Value{
        Value::U64(self.clone())
    }
}

impl ToValue for f32{
    fn to_db_type(&self)->Value{
        Value::F32(self.clone())
    }
}

impl ToValue for f64{
    fn to_db_type(&self)->Value{
        Value::F64(self.clone())
    }
}

impl <'a>ToValue for &'a str{
    fn to_db_type(&self)->Value{
        Value::String(self.to_string())
    }
}

impl ToValue for String{
    fn to_db_type(&self)->Value{
        Value::String(self.clone())
    }
}

impl ToValue for Uuid{
    fn to_db_type(&self)->Value{
        Value::Uuid(self.clone())
    }
}

impl ToValue for DateTime<UTC>{
    fn to_db_type(&self)->Value{
        Value::DateTime(self.clone())
    }
}

impl ToValue for NaiveDate{
    fn to_db_type(&self)->Value{
        Value::NaiveDate(self.clone())
    }
}

impl ToValue for NaiveTime{
    fn to_db_type(&self)->Value{
        Value::NaiveTime(self.clone())
    }
}
impl ToValue for NaiveDateTime{
    fn to_db_type(&self)->Value{
        Value::NaiveDateTime(self.clone())
    }
}

///
///
///
///
///
///
///
///
///
///
///
///
///
pub trait FromValue{
    fn from_type(ty:Value)->Self;
}

impl FromValue for bool{
    fn from_type(ty:Value)->Self{
        match ty{
            Value::Bool(x) => x,
            _ => panic!("error!"),
        }
    }
}

impl FromValue for i8{
    fn from_type(ty:Value)->Self{
        match ty{
            Value::I8(x) => x,
            _ => panic!("error!"),
        }
    }
}
impl FromValue for i16{
    fn from_type(ty:Value)->Self{
        match ty{
            Value::I16(x) => x,
            _ => panic!("error!"),
        }
    }
}
impl FromValue for i32{
    fn from_type(ty:Value)->Self{
        match ty{
            Value::I32(x) => x,
            _ => panic!("error!"),
        }
    }
}
impl FromValue for i64{
    fn from_type(ty:Value)->Self{
        match ty{
            Value::I64(x) => x,
            _ => panic!("error!"),
        }
    }
}
impl FromValue for u8{
    fn from_type(ty:Value)->Self{
        match ty{
            Value::U8(x) => x,
            _ => panic!("error!"),
        }
    }
}
impl FromValue for u16{
    fn from_type(ty:Value)->Self{
        match ty{
            Value::U16(x) => x,
            _ => panic!("error!"),
        }
    }
}
impl FromValue for u32{
    fn from_type(ty:Value)->Self{
        match ty{
            Value::U32(x) => x,
            _ => panic!("error!"),
        }
    }
}
impl FromValue for u64{
    fn from_type(ty:Value)->Self{
        match ty{
            Value::U64(x) => x,
            _ => panic!("error!"),
        }
    }
}

impl FromValue for f32{
    fn from_type(ty:Value)->Self{
        match ty{
            Value::F32(x) => x,
            _ => panic!("error!"),
        }
    }
}
impl FromValue for f64{
    fn from_type(ty:Value)->Self{
        match ty{
            Value::F64(x) => x,
            _ => panic!("error! {:?}",ty),
        }
    }
}

impl FromValue for String{
    fn from_type(ty:Value)->Self{
        match ty{
            Value::String(x) => x,
            _ => panic!("error! {:?}", ty),
        }
    }
}

impl FromValue for Uuid{
    fn from_type(ty:Value)->Self{
        match ty{
            Value::Uuid(x) => x,
            Value::Null => panic!("No value!"),
            _ => panic!("error!"),
        }
    }
}

impl FromValue for DateTime<UTC>{
    fn from_type(ty:Value)->Self{
        match ty{
            Value::DateTime(x) => x,
            _ => panic!("error!"),
        }
    }
}

impl FromValue for NaiveTime{
    fn from_type(ty:Value)->Self{
        match ty{
            Value::NaiveTime(x) => x,
            _ => panic!("error!"),
        }
    }
}

impl FromValue for NaiveDate{
    fn from_type(ty:Value)->Self{
        match ty{
            Value::NaiveDate(x) => x,
            _ => panic!("error!"),
        }
    }
}

impl FromValue for NaiveDateTime{
    fn from_type(ty:Value)->Self{
        match ty{
            Value::NaiveDateTime(x) => x,
            _ => panic!("error!"),
        }
    }
}



#[test]
fn test_dao(){
    let s = "lee";
    let n = 20i8;
    let date = UTC::now();
    let mut d = Dao::new();
    d.set("name", &s);
    d.set("age", &n);
    d.set("created", &date);
    let name:String = d.get("name");
    let age:i8 = d.get("age");
    let created:DateTime<UTC> = d.get("created");
    let none:Option<u8> = d.get_opt("none");
    assert_eq!(name, s);
    assert_eq!(age, 20i8);
    assert_eq!(date, created);
    assert_eq!(none, None);
}

#[test]
fn test_json(){
    let s = "lee";
    let n = 20i8;
    let date = UTC::now();
    let mut dao = Dao::new();
    dao.set("name", &s);
    dao.set("age", &n);
    dao.set("created", &date);
    let name:String = dao.get("name");
    let age:i8 = dao.get("age");
    let created:DateTime<UTC> = dao.get("created");
    let none:Option<u8> = dao.get_opt("none");
    let expected = r#"{"age":20,"created":{"datetime":{"date":{"ymdf":16510090},"time":{"secs":28087,"frac":451865185}},"offset":{}},"name":"lee"}"#;
    let actual = json::encode(&dao).unwrap();
    println!("expected: {}", expected);
    println!("actual: {}",actual);
}
