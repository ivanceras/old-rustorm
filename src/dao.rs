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
use rustc_serialize::{Encodable, Encoder, Decodable, Decoder};
use rustc_serialize::json::{self, ToJson, Json};
use rustc_serialize::DecoderHelpers;
use rustc_serialize::base64::STANDARD;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64::FromBase64;
use chrono::offset::fixed::FixedOffset;


#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(RustcEncodable)]
/// supported generic datatypes for an ORM
pub enum Type {
    Bool,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    String,
    VecU8,
    Json,
    Uuid,
    DateTime,
    NaiveDate,
    NaiveTime,
    NaiveDateTime,
}
impl Type {
    /// get the string representation when used in rust code
    pub fn to_str_repr(&self) -> String {
        match *self {
            Type::Bool => "bool".to_owned(),
            Type::I8 => "i8".to_owned(),
            Type::I16 => "i16".to_owned(),
            Type::I32 => "i32".to_owned(),
            Type::I64 => "i64".to_owned(),
            Type::U8 => "u8".to_owned(),
            Type::U16 => "u16".to_owned(),
            Type::U32 => "u32".to_owned(),
            Type::U64 => "u64".to_owned(),
            Type::F32 => "f32".to_owned(),
            Type::F64 => "f64".to_owned(),
            Type::String => "String".to_owned(),
            Type::VecU8 => "Vec<u8>".to_owned(),
            Type::Json => "Json".to_owned(),
            Type::Uuid => "Uuid".to_owned(),
            Type::DateTime => "DateTime<UTC>".to_owned(),
            Type::NaiveDate => "NaiveDate".to_owned(),
            Type::NaiveTime => "NaiveTime".to_owned(),
            Type::NaiveDateTime => "NaiveDateTime".to_owned(),

        }
    }
}


#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
/// supported generic datatypes for an ORM
pub enum Value {
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
    Json(Json),
    Uuid(Uuid),
    DateTime(DateTime<FixedOffset>),
    NaiveDate(NaiveDate),
    NaiveTime(NaiveTime),
    NaiveDateTime(NaiveDateTime),
}

impl Value {
    pub fn get_type(&self) -> Type {
        match *self {
            Value::Bool(_) => Type::Bool,
            Value::I8(_) => Type::I8,
            Value::I16(_) => Type::I16,
            Value::I32(_) => Type::I32,
            Value::I64(_) => Type::I64,
            Value::U8(_) => Type::U8,
            Value::U16(_) => Type::U16,
            Value::U32(_) => Type::U32,
            Value::U64(_) => Type::U64,
            Value::F32(_) => Type::F32,
            Value::F64(_) => Type::F64,
            Value::String(_) => Type::String,
            Value::VecU8(_) => Type::VecU8,
            Value::Uuid(_) => Type::Uuid,
            Value::DateTime(_) => Type::DateTime,
            Value::NaiveDate(_) => Type::NaiveDate,
            Value::NaiveTime(_) => Type::NaiveTime,
            Value::NaiveDateTime(_) => Type::NaiveDateTime,
            Value::Json(_) => Type::Json,
        }
    }

    fn from_ser_value(ser_value: &SerValue) -> Self {
        match ser_value {
            &SerValue::Bool(x) => Value::Bool(x),
            &SerValue::I8(x) => Value::I8(x),
            &SerValue::I16(x) => Value::I16(x),
            &SerValue::I32(x) => Value::I32(x),
            &SerValue::I64(x) => Value::I64(x),
            &SerValue::U8(x) => Value::U8(x),
            &SerValue::U16(x) => Value::U16(x),
            &SerValue::U32(x) => Value::U32(x),
            &SerValue::U64(x) => Value::U64(x),
            &SerValue::F32(x) => Value::F32(x),
            &SerValue::F64(x) => Value::F64(x),
            &SerValue::String(ref x) => Value::String(x.to_owned()),
            &SerValue::VecU8(ref x) => {
                let vecu8 = x.from_base64().unwrap();
                Value::VecU8(vecu8)
            }
            &SerValue::Uuid(x) => Value::Uuid(x),
            &SerValue::DateTime(ref x) => {
                let date = DateTime::parse_from_rfc3339(x).unwrap();
                Value::DateTime(date)
            }
            &SerValue::NaiveDate(ref x) => {
                // let date = DateTime::parse_from_str(x);
                // Value::NaiveDate(date)
                panic!("not yet here!");
            }
            &SerValue::NaiveTime(ref x) => {
                // let time = NaiveTime::parse_from_str(x);
                // Value::NaiveTime(time)
                panic!("not yet here!");
            }
            &SerValue::NaiveDateTime(ref x) => {
                // let time = NaiveTime::parse_from_str(x);
                // Value::NaiveTime(time)
                panic!("not yet here!");
            }
            &SerValue::Json(ref json) => {
                let json = Json::from_str(json).unwrap();
                Value::Json(json)
            }
        }
    }
}



/// custom implementation for value encoding to json,
/// does not include unnecessary enum variants fields.
impl Encodable for Value {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        let ser_value = SerValue::from_value(&self);
        ser_value.encode(s)
    }
}

/// serializable value to json, to avoid complexity
/// in manipulating the clienside json things such as date,
#[derive(RustcEncodable)]
#[derive(RustcDecodable)]
enum SerValue {
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
    VecU8(String), // blob in 64 bit
    Uuid(Uuid),
    DateTime(String), // in standard format string
    NaiveDate(String),
    NaiveTime(String),
    NaiveDateTime(String),
    Json(String),
}

impl SerValue {
    fn from_value(value: &Value) -> Self {
        match value {
            &Value::Bool(x) => SerValue::Bool(x),
            &Value::I8(x) => SerValue::I8(x),
            &Value::I16(x) => SerValue::I16(x),
            &Value::I32(x) => SerValue::I32(x),
            &Value::I64(x) => SerValue::I64(x),
            &Value::U8(x) => SerValue::U8(x),
            &Value::U16(x) => SerValue::U16(x),
            &Value::U32(x) => SerValue::U32(x),
            &Value::U64(x) => SerValue::U64(x),
            &Value::F32(x) => SerValue::F32(x),
            &Value::F64(x) => SerValue::F64(x),
            &Value::String(ref x) => SerValue::String(x.to_owned()),
            &Value::VecU8(ref x) => {
                let base64 = x.to_base64(STANDARD);
                SerValue::VecU8(base64)
            }
            &Value::Uuid(x) => SerValue::Uuid(x),
            &Value::DateTime(ref x) => {
                let date_str = x.to_rfc3339();
                SerValue::DateTime(date_str)
            }
            &Value::NaiveDate(ref x) => {
                let date_str = format!("{}", x);
                SerValue::NaiveDate(date_str)
            }
            &Value::NaiveTime(ref x) => {
                let time_str = format!("{}", x);
                SerValue::NaiveTime(time_str)
            }
            &Value::NaiveDateTime(ref x) => {
                let time_str = format!("{}", x);
                SerValue::NaiveTime(time_str)
            }
            &Value::Json(ref json) => {
                let json_text = format!("{}", json.pretty());
                SerValue::Json(json_text)
            }
        }

    }
}

/// A quick solution to controlling the output of the decoded
/// json value at right amount of data structure nesting...
impl Decodable for Value {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        let ser_value = try!(SerValue::decode(d));
        let value = Value::from_ser_value(&ser_value);
        Ok(value)
    }
}

#[test]
fn test_decode_value() {
    let mut dao = Dao::new();
    dao.insert("hello".to_owned(), Value::String("hi".to_owned()));
    let dao_json = json::encode(&dao).unwrap();
    println!("{:#?}", dao_json);
    let dec: Dao = json::decode(&dao_json).unwrap();
    println!("{:#?}", dec);
    assert_eq!(dao, dec);
}



impl fmt::Display for Value {
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
            Value::F32(ref x) => write!(f, "'{}'", x),
            Value::F64(ref x) => write!(f, "'{}'", x),
            Value::String(ref x) => write!(f, "'{}'", x),
            Value::VecU8(ref x) => write!(f, "'{:?}'", x),
            Value::Uuid(ref x) => write!(f, "'{}'", x),
            Value::DateTime(ref x) => write!(f, "'{}'", x),
            Value::NaiveDate(ref x) => write!(f, "'{}'", x),
            Value::NaiveTime(ref x) => write!(f, "'{}'", x),
            Value::NaiveDateTime(ref x) => write!(f, "'{}'", x),
            Value::Json(ref x) => write!(f, "'{:?}'", x),
        }
    }
}


/// trait for converting dao to model
/// sized and clonable
pub trait IsDao {
    /// convert dao to an instance of the corresponding struct of the model
    /// taking into considerating the renamed columns
    fn from_dao(dao: &Dao) -> Self;

    /// convert from an instance of the struct to a dao representation
    /// to be saved into the database
    fn to_dao(&self) -> Dao;
}

/// Ignore Column are columns that are redundant when displaying as API results
pub trait ToCompact {
    /// list of redundant fields that will be removed when doing a compact serialization
    fn redundant_fields(&self) -> Vec<&str>;

    /// compact BTreeMap represetation
    fn compact_map(&self) -> BTreeMap<String, Value>;

    /// compact dao representation
    fn compact_dao(&self) -> Dao;

    /// compact dao representation
    fn compact_json(&self) -> Json;
}

/// meta result of a query useful when doing complex query, and also with paging
/// TODO: good name: DaoRows
#[derive(Debug,Clone)]
#[derive(RustcEncodable)]
#[derive(RustcDecodable)]
pub struct DaoResult {
    pub dao: Vec<Dao>,
    /// renamed columns for each table
    /// ie. product => [(name, product_name),..];
    pub renamed_columns: Vec<(ColumnName, String)>,

    /// the total number of records
    pub total: Option<usize>,
    /// page of the query
    pub page: Option<usize>,
    /// page size
    pub page_size: Option<usize>,
}


impl DaoResult {
    /// get the list of renamed column name in matching table name
    fn get_renamed_columns(&self, table: &str) -> Vec<(String, String)> {
        let mut columns = vec![];
        for &(ref col, ref rename) in &self.renamed_columns {
            if let Some(ref s) = col.table {
                if s == table {
                    columns.push((col.column.to_owned(), rename.to_owned()));
                }
            }
        }
        columns
    }

    /// cast the dao to the specific struct instance
    /// do not include if non nullable parts contains null
    pub fn cast<T: IsTable + IsDao>(&self) -> Vec<T> {
        let table = T::table();
        let non_nulls = table.non_nullable_columns();
        let mut obj = vec![];
        let renamed_columns = self.get_renamed_columns(&table.name);
        for dao in &self.dao {
            let mut dao_clone = dao.clone();
            dao_clone.correct_renamed_columns(&renamed_columns);
            if dao_clone.all_has_values(&non_nulls) {
                let p = T::from_dao(&dao_clone);
                obj.push(p);
            }
        }
        obj
    }

    /// FIXME: should return an error when there are more than 1 to be casted
    pub fn cast_one<T: IsTable + IsDao>(&self) -> Option<T> {
        let mut casted = self.cast::<T>();
        if casted.len() < 1 {
            return None;
        }
        Some(casted.remove(0))
    }
}

/// TODO: optimization, used enum types for the key values
/// This will save allocation of string to enum keys which is a few bytes, int
pub type Dao = BTreeMap<String, Value>;


pub type ParseError = String;


trait DaoCorrections {
    fn correct_renamed_columns(&mut self, renamed_columns: &Vec<(String, String)>);

    fn all_has_values(&self, non_nulls: &Vec<String>) -> bool;
}

impl DaoCorrections for Dao {
    fn correct_renamed_columns(&mut self, renamed_columns: &Vec<(String, String)>) {
        for &(ref column, ref rename) in renamed_columns {
            let value: Option<Value> = match self.get(rename) {
                Some(value) => Some(value.to_owned()),
                None => None,
            };
            if let Some(value) = value {
                self.insert(column.to_owned(), value.clone());
            }
        }
    }

    fn all_has_values(&self, non_nulls: &Vec<String>) -> bool {
        for column in non_nulls {
            let value = self.get(column);
            if let Some(value) = value {
                // has value
            } else {
                return false;
            }
        }
        true
    }
}


/// rename to ToValue
pub trait ToValue {
    fn to_db_type(&self) -> Value;
}

impl ToValue for Value {
    fn to_db_type(&self) -> Value {
        self.to_owned()
    }
}


impl ToValue for bool {
    fn to_db_type(&self) -> Value {
        Value::Bool(self.clone())
    }
}

/// signed INTs
impl ToValue for i8 {
    fn to_db_type(&self) -> Value {
        Value::I8(self.clone())
    }
}

impl ToValue for i16 {
    fn to_db_type(&self) -> Value {
        Value::I16(self.clone())
    }
}
impl ToValue for i32 {
    fn to_db_type(&self) -> Value {
        Value::I32(self.clone())
    }
}

impl ToValue for i64 {
    fn to_db_type(&self) -> Value {
        Value::I64(self.clone())
    }
}
/// unsigned INTs
impl ToValue for u8 {
    fn to_db_type(&self) -> Value {
        Value::U8(self.clone())
    }
}

impl ToValue for u16 {
    fn to_db_type(&self) -> Value {
        Value::U16(self.clone())
    }
}
impl ToValue for u32 {
    fn to_db_type(&self) -> Value {
        Value::U32(self.clone())
    }
}

impl ToValue for u64 {
    fn to_db_type(&self) -> Value {
        Value::U64(self.clone())
    }
}

impl ToValue for f32 {
    fn to_db_type(&self) -> Value {
        Value::F32(self.clone())
    }
}

impl ToValue for f64 {
    fn to_db_type(&self) -> Value {
        Value::F64(self.clone())
    }
}

impl ToValue for String {
    fn to_db_type(&self) -> Value {
        Value::String(self.clone())
    }
}

impl ToValue for Uuid {
    fn to_db_type(&self) -> Value {
        Value::Uuid(self.clone())
    }
}

impl ToValue for DateTime<FixedOffset> {
    fn to_db_type(&self) -> Value {
        Value::DateTime(self.clone())
    }
}

impl ToValue for NaiveDate {
    fn to_db_type(&self) -> Value {
        Value::NaiveDate(self.clone())
    }
}

impl ToValue for NaiveTime {
    fn to_db_type(&self) -> Value {
        Value::NaiveTime(self.clone())
    }
}
impl ToValue for NaiveDateTime {
    fn to_db_type(&self) -> Value {
        Value::NaiveDateTime(self.clone())
    }
}

impl ToValue for Json {
    fn to_db_type(&self) -> Value {
        Value::Json(self.clone())
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
pub trait FromValue {
    fn from_type(ty: Value) -> Self;
}

impl FromValue for bool {
    fn from_type(ty: Value) -> Self {
        match ty {
            Value::Bool(x) => x,
            _ => panic!("error!"),
        }
    }
}

impl FromValue for i8 {
    fn from_type(ty: Value) -> Self {
        match ty {
            Value::I8(x) => x,
            _ => panic!("error!"),
        }
    }
}
impl FromValue for i16 {
    fn from_type(ty: Value) -> Self {
        match ty {
            Value::I16(x) => x,
            _ => panic!("error!"),
        }
    }
}
impl FromValue for i32 {
    fn from_type(ty: Value) -> Self {
        match ty {
            Value::I32(x) => x,
            _ => panic!("error!"),
        }
    }
}
impl FromValue for i64 {
    fn from_type(ty: Value) -> Self {
        match ty {
            Value::I64(x) => x,
            _ => panic!("error!"),
        }
    }
}
impl FromValue for u8 {
    fn from_type(ty: Value) -> Self {
        match ty {
            Value::U8(x) => x,
            _ => panic!("error!"),
        }
    }
}
impl FromValue for u16 {
    fn from_type(ty: Value) -> Self {
        match ty {
            Value::U16(x) => x,
            _ => panic!("error!"),
        }
    }
}
impl FromValue for u32 {
    fn from_type(ty: Value) -> Self {
        match ty {
            Value::U32(x) => x,
            _ => panic!("error!"),
        }
    }
}
impl FromValue for u64 {
    fn from_type(ty: Value) -> Self {
        match ty {
            Value::U64(x) => x,
            _ => panic!("error!"),
        }
    }
}

impl FromValue for f32 {
    fn from_type(ty: Value) -> Self {
        match ty {
            Value::F32(x) => x,
            _ => panic!("error!"),
        }
    }
}
impl FromValue for f64 {
    fn from_type(ty: Value) -> Self {
        match ty {
            Value::F64(x) => x,
            _ => panic!("error! {:?}", ty),
        }
    }
}

impl FromValue for String {
    fn from_type(ty: Value) -> Self {
        match ty {
            Value::String(x) => x,
            _ => panic!("error! {:?}", ty),
        }
    }
}

impl FromValue for Uuid {
    fn from_type(ty: Value) -> Self {
        match ty {
            Value::Uuid(x) => x,
            _ => panic!("error!"),
        }
    }
}

impl FromValue for DateTime<FixedOffset> {
    fn from_type(ty: Value) -> Self {
        match ty {
            Value::DateTime(x) => x,
            _ => panic!("error!"),
        }
    }
}

impl FromValue for NaiveTime {
    fn from_type(ty: Value) -> Self {
        match ty {
            Value::NaiveTime(x) => x,
            _ => panic!("error!"),
        }
    }
}

impl FromValue for NaiveDate {
    fn from_type(ty: Value) -> Self {
        match ty {
            Value::NaiveDate(x) => x,
            _ => panic!("error!"),
        }
    }
}

impl FromValue for NaiveDateTime {
    fn from_type(ty: Value) -> Self {
        match ty {
            Value::NaiveDateTime(x) => x,
            _ => panic!("error!"),
        }
    }
}

impl FromValue for Json {
    fn from_type(ty: Value) -> Self {
        match ty {
            Value::Json(x) => x,
            _ => panic!("error!"),
        }
    }
}
