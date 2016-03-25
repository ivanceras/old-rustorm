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


#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
///supported generic datatypes for an ORM
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
    Object,
    Json,
    Uuid,
    DateTime,
    NaiveDate,
    NaiveTime,
    NaiveDateTime,
	None,
}
impl Type{
	/// get the string representation when used in rust code
	pub fn to_str_repr(&self)->String{
		match *self{
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
			Type::String =>"String".to_owned(),
			Type::VecU8 => "Vec<u8>".to_owned(),
			Type::Object => "BTreeMap<String, Value>".to_owned(), 
			Type::Json => "Json".to_owned(),
			Type::Uuid => "Uuid".to_owned(),
			Type::DateTime => "DateTime<UTC>".to_owned(),
			Type::NaiveDate => "NaiveDate".to_owned(),
			Type::NaiveTime => "NaiveTime".to_owned(),
			Type::NaiveDateTime => "NaiveDateTime".to_owned(),
			Type::None => "None".to_owned(),

		}
	}
}


#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
///supported generic datatypes for an ORM
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
    Object(BTreeMap<String, Value>),
    Json(Json),
    Uuid(Uuid),
    DateTime(DateTime<UTC>),
    NaiveDate(NaiveDate),
    NaiveTime(NaiveTime),
    NaiveDateTime(NaiveDateTime),
    None(Type),
}

impl Value{
	
	pub fn get_type(&self)->Type{
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
            Value::Object(_) => Type::Object,
            Value::Json(_) => Type::Json,
            Value::None(_) => Type::None,
        }
	}
}


/// custom implementation for value encoding to json,
/// does not include unnecessary enum variants fields.
impl Encodable for Value {

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
            Value::DateTime(ref x) => {
                x.to_rfc3339().encode(s)
            }
            Value::NaiveDate(ref x) => x.encode(s),
            Value::NaiveTime(ref x) => x.encode(s),
            Value::NaiveDateTime(ref x) => x.encode(s),
            Value::Object(ref x) => x.encode(s),
            Value::Json(ref x) => x.encode(s),
            Value::None(_) => s.emit_nil(),
        }
    }
}

impl ToJson for Value {

    fn to_json(&self) -> Json {
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
            Value::Object(ref x) => x.to_json(),
            Value::Json(ref x) => x.clone(),
            Value::None(_) => Json::Null,
            _ => panic!("unsupported/unexpected type! {:?}", self),
        }
    }
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
            Value::String(ref x) => write!(f, "'{}'", x),
            Value::VecU8(ref x) => write!(f, "'{:?}'", x),
            Value::Uuid(ref x) => write!(f, "'{}'", x),
            Value::DateTime(ref x) => write!(f, "'{}'", x),
            Value::NaiveDate(ref x) => write!(f, "'{}'", x),
            Value::NaiveTime(ref x) => write!(f, "'{}'", x),
            Value::NaiveDateTime(ref x) => write!(f, "'{}'", x),
            Value::Object(ref x) => write!(f, "'{:?}'", x),
            Value::Json(ref x) => write!(f, "'{:?}'", x),
            Value::None(_) => write!(f, "'nil'"),
            _ => panic!("unsupported/unexpected type! {:?}", self),
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
#[derive(Debug)]
pub struct DaoResult {
    pub dao: Vec<Dao>,
    ///renamed columns for each table
    /// ie. product => [(name, product_name),..];
    pub renamed_columns: Vec<(ColumnName, String)>,

    /// the total number of records
    pub total: Option<usize>,
    /// page of the query
    pub page: Option<usize>,
    /// page size
    pub page_size: Option<usize>,
}

/// a serializable array of dao to be serialized to json request
/// a utility struct to hold only the needed fields from DaoResult that is needed
/// in serializing the object, the non-significant ones are not included such as the renamed_columns
/// TODO: add Decodable
#[derive(RustcEncodable)]
pub struct SerDaoResult {
    pub dao: Vec<Dao>,
    pub total: Option<usize>,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}

impl SerDaoResult {

    pub fn from_dao_result(daoresult: &DaoResult) -> Self {
        SerDaoResult {
            dao: daoresult.dao.clone(),
            total: daoresult.total.clone(),
            page: daoresult.page.clone(),
            page_size: daoresult.page_size.clone(),
        }
    }
}

impl Encodable for DaoResult {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
		let ser = SerDaoResult::from_dao_result(self);
		ser.encode(s)
    }
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
	// why here assert??!!!
        //assert!(casted.len() == 1);
        Some(casted.remove(0))
    }
}

#[derive(Debug, Clone)]
/// TODO: optimization, used enum types for the key values
/// This will save allocation of string to enum keys which is a few bytes, int
pub struct Dao {
    pub values: BTreeMap<String, Value>,
}

pub type ParseError = String;

impl Dao{
	pub fn from_str(s: &str)->Result<Vec<Self>, ParseError>{
		debug!("parsing multiple records from json");
		let json: Json = Json::from_str(s).unwrap();
		debug!("from str json: {:#?}", json);
		match json{
			Json::Array(array) => {
				let mut dao_list = vec![];
				for obj in array{
					let map = Self::json_object_to_btree(obj);
					let dao = Dao{ values: map };
					dao_list.push(dao);
				}
				Ok(dao_list)
			},
			_ => Err("Expecting an array".to_owned())
		}
	}

	fn json_object_to_btree(json: Json)->BTreeMap<String, Value>{
		match json{
			Json::Object(btree) => {
				let mut new_map:BTreeMap<String, Value> = BTreeMap::new();
				for (k, v) in btree.iter(){
					let value = match v{
						&Json::I64(v) => Value::I64(v),
						&Json::U64(v) => Value::U64(v),
						&Json::F64(v) => Value::F64(v),
						&Json::String(ref v) => Value::String(v.to_owned()),
						&Json::Boolean(v) => Value::Bool(v),
						&Json::Null => Value::None(Type::Json),
						&Json::Object(ref v) => {
							let mut map: BTreeMap<String, Value> = BTreeMap::new();
							for (k, v) in v.iter(){
								let value = Value::Json(v.clone());
								map.insert(k.to_owned(), value);
							}
							Value::Object(map)
						},
						&Json::Array(ref arr) => {
							Value::Json(Json::Array(arr.clone()))
						},
					};
					new_map.insert(k.to_owned(), value);
				}
				new_map
			},
			_ => panic!("expecting an object"),
		}
	}
	/// reconstruct a dao from json string value
	pub fn from_str_one(s: &str)->Result<Self, ()>{
		let json: Json = Json::from_str(s).unwrap();
		// then convert this map to Value
		debug!("from str: {:#?}", json);
		let values = Self::json_object_to_btree(json);
		Ok(Dao{
			values: values
		})
	}
}

/// custom Encoder for Dao,
/// decodes directly the content of `values`, instead of `values` as field of this `Dao` struct
impl Encodable for Dao {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        self.values.encode(s)
    }
}
impl ToJson for Dao {

    fn to_json(&self) -> Json {
        let mut btree = BTreeMap::new();
        for (key, value) in &self.values {
            btree.insert(key.to_owned(), value.to_json());
        }
        Json::Object(btree)
    }

}

impl Dao {

    pub fn new() -> Self {
        Dao { values: BTreeMap::new() }
    }

    pub fn set(&mut self, column: &str, value: &ToValue) {
        self.values.insert(column.to_owned(), value.to_db_type());
    }

    /// set to null the value of this column
	/// TODO: correct the Null value here
    pub fn set_null(&mut self, column: &str) {
        self.set_value(column, Value::None(Type::String))
    }

    pub fn set_value(&mut self, column: &str, value: Value) {
        self.values.insert(column.to_owned(), value);
    }
    pub fn get_value(&self, column: &str) -> Value {
        let value = self.values.get(column);
        match value {
            Some(value) => value.clone(),
            None => panic!("No such value for {}", column),
        }
    }
    /// take the value and remove the content
    pub fn remove<T>(&mut self, column: &str) -> T
        where T: FromValue
    {
        let value = self.values.remove(column).unwrap();
        FromValue::from_type(value)
    }

    /// take the value but not removing the content
    pub fn get<T>(&self, column: &str) -> T
        where T: FromValue
    {
        let value = self.values.get(column).unwrap();
        FromValue::from_type(value.clone())
    }
    /// get optional value
    pub fn get_opt<T>(&self, column: &str) -> Option<T>
        where T: FromValue
    {
        let value = self.values.get(column);
        match value {
            None | Some(&Value::None(_)) => None,
            Some(v) => Some(FromValue::from_type(v.clone())),
        }
    }

    /// get a reference of the type
    pub fn as_ref(&self, column: &str) -> &Value {
        self.values.get(column).unwrap()
    }


    fn correct_renamed_columns(&mut self, renamed_columns: &Vec<(String, String)>) {
        for &(ref column, ref rename) in renamed_columns {
            let value = self.get_value(rename);
            self.set_value(&column, value);
        }
    }

    fn all_has_values(&self, non_nulls: &Vec<String>) -> bool {
        for column in non_nulls {
            let value = self.values.get(column);
            match value {
                None | Some(&Value::None(_)) => return false,
                _ => (),
            }
        }
        true
    }


    pub fn as_map(&self) -> &BTreeMap<String, Value> {
        &self.values
    }
}


/// rename to ToValue
pub trait ToValue {
    fn to_db_type(&self) -> Value;
}

impl ToValue for Value{
	fn to_db_type(&self)->Value{
		self.to_owned()
	}
}

impl ToValue for () {
    fn to_db_type(&self) -> Value {
        Value::None(Type::String)
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
/*
impl <'a>ToValue for &'a str {
    fn to_db_type(&self) -> Value {
        Value::String((*self).to_owned())
    }
}
*/

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

impl ToValue for DateTime<UTC> {
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

impl FromValue for DateTime<UTC> {
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

