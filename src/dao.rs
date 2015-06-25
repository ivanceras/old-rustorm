use std::collections::HashMap;
use std::collections::BTreeMap;
use uuid::Uuid;
use chrono::datetime::DateTime;
use chrono::naive::date::NaiveDate;
use chrono::naive::time::NaiveTime;
use chrono::naive::datetime::NaiveDateTime;
use chrono::offset::utc::UTC;

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
#[derive(Clone)]
///supported generic datatypes for an ORM
pub enum Type{
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

pub trait ToType{
    fn to_db_type(&self)->Type;
}
/// signed INTs
impl ToType for i8{
    fn to_db_type(&self)->Type{
        Type::I8(self.clone())
    }
}

impl ToType for i16{
    fn to_db_type(&self)->Type{
        Type::I16(self.clone())
    }
}
impl ToType for i32{
    fn to_db_type(&self)->Type{
        Type::I32(self.clone())
    }
}

impl ToType for i64{
    fn to_db_type(&self)->Type{
        Type::I64(self.clone())
    }
}
/// unsigned INTs
impl ToType for u8{
    fn to_db_type(&self)->Type{
        Type::U8(self.clone())
    }
}

impl ToType for u16{
    fn to_db_type(&self)->Type{
        Type::U16(self.clone())
    }
}
impl ToType for u32{
    fn to_db_type(&self)->Type{
        Type::U32(self.clone())
    }
}

impl ToType for u64{
    fn to_db_type(&self)->Type{
        Type::U64(self.clone())
    }
}

impl <'a>ToType for &'a str{
    fn to_db_type(&self)->Type{
        Type::String(self.to_string())
    }
}

impl ToType for String{
    fn to_db_type(&self)->Type{
        Type::String(self.clone())
    }
}

impl ToType for Uuid{
    fn to_db_type(&self)->Type{
        Type::Uuid(self.clone())
    }
}

impl ToType for DateTime<UTC>{
    fn to_db_type(&self)->Type{
        Type::DateTime(self.clone())
    }
}

impl ToType for NaiveDate{
    fn to_db_type(&self)->Type{
        Type::NaiveDate(self.clone())
    }
}

impl ToType for NaiveTime{
    fn to_db_type(&self)->Type{
        Type::NaiveTime(self.clone())
    }
}
impl ToType for NaiveDateTime{
    fn to_db_type(&self)->Type{
        Type::NaiveDateTime(self.clone())
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
///
pub trait FromType{
    fn from_type(ty:Type)->Self;
}

impl FromType for bool{
    fn from_type(ty:Type)->Self{
        match ty{
            Type::Bool(x) => x,
            _ => panic!("error!"),
        }
    }
}

impl FromType for i8{
    fn from_type(ty:Type)->Self{
        match ty{
            Type::I8(x) => x,
            _ => panic!("error!"),
        }
    }
}
impl FromType for i16{
    fn from_type(ty:Type)->Self{
        match ty{
            Type::I16(x) => x,
            _ => panic!("error!"),
        }
    }
}
impl FromType for i32{
    fn from_type(ty:Type)->Self{
        match ty{
            Type::I32(x) => x,
            _ => panic!("error!"),
        }
    }
}
impl FromType for i64{
    fn from_type(ty:Type)->Self{
        match ty{
            Type::I64(x) => x,
            _ => panic!("error!"),
        }
    }
}
impl FromType for u8{
    fn from_type(ty:Type)->Self{
        match ty{
            Type::U8(x) => x,
            _ => panic!("error!"),
        }
    }
}
impl FromType for u16{
    fn from_type(ty:Type)->Self{
        match ty{
            Type::U16(x) => x,
            _ => panic!("error!"),
        }
    }
}
impl FromType for u32{
    fn from_type(ty:Type)->Self{
        match ty{
            Type::U32(x) => x,
            _ => panic!("error!"),
        }
    }
}
impl FromType for u64{
    fn from_type(ty:Type)->Self{
        match ty{
            Type::U64(x) => x,
            _ => panic!("error!"),
        }
    }
}

impl FromType for f32{
    fn from_type(ty:Type)->Self{
        match ty{
            Type::F32(x) => x,
            _ => panic!("error!"),
        }
    }
}
impl FromType for f64{
    fn from_type(ty:Type)->Self{
        match ty{
            Type::F64(x) => x,
            _ => panic!("error!"),
        }
    }
}

impl FromType for String{
    fn from_type(ty:Type)->Self{
        match ty{
            Type::String(x) => x,
            _ => panic!("error! {:?}", ty),
        }
    }
}

impl FromType for Uuid{
    fn from_type(ty:Type)->Self{
        match ty{
            Type::Uuid(x) => x,
            _ => panic!("error!"),
        }
    }
}

impl FromType for DateTime<UTC>{
    fn from_type(ty:Type)->Self{
        match ty{
            Type::DateTime(x) => x,
            _ => panic!("error!"),
        }
    }
}

impl FromType for NaiveTime{
    fn from_type(ty:Type)->Self{
        match ty{
            Type::NaiveTime(x) => x,
            _ => panic!("error!"),
        }
    }
}

impl FromType for NaiveDate{
    fn from_type(ty:Type)->Self{
        match ty{
            Type::NaiveDate(x) => x,
            _ => panic!("error!"),
        }
    }
}

impl FromType for NaiveDateTime{
    fn from_type(ty:Type)->Self{
        match ty{
            Type::NaiveDateTime(x) => x,
            _ => panic!("error!"),
        }
    }
}

/// trait for converting dao to model
pub trait IsDao:Sized{
    
    /// converts a vector of dao to an object
    fn from_dao_result(dao_result:&DaoResult)->Vec<Self>{
        let mut obj = vec![];
        for d in &dao_result.dao{
            let p = Self::from_dao_with_renamed(d, &dao_result.renamed_columns);
            obj.push(p);
        }
        obj
    }
    /// convert a dao object into the an instance of the generated struct from table
    /// taking into considerating the renamed columns
    fn from_dao_with_renamed(dao:&Dao, renamed_columns:&BTreeMap<String, Vec<(String, String)>>)->Self{
        let mut dao = dao.clone();
        dao.resolve_renamed_columns("product", renamed_columns);
        Self::from_dao(&mut dao)
    }
    fn from_dao(dao: &Dao)->Self;
}

/// meta result of a query useful when doing complex query, and also with paging
pub struct DaoResult{
    /// an optional table where this Dao values was derived from
    pub from_table:Option<String>,
    pub dao: Vec<Dao>,
    ///renamed columns for each table
    /// ie. product => [(name, product_name),..];
    pub renamed_columns: BTreeMap<String, Vec<(String, String)>>, 
    
    /// the total number of records
    pub total:Option<usize>,
    /// page of the query
    pub page: Option<usize>,
    /// page size
    pub page_size: Option<usize>,
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug, Clone)]
pub struct Dao{
    pub values:HashMap<String, Type>,
}

impl Dao{

    pub fn new()->Self{
        Dao{values:HashMap::new()}
    }
    
    pub fn set(&mut self, column: &str, value:&ToType){
        self.values.insert(column.to_string(), value.to_db_type());
    }
    
    pub fn set_value(&mut self, column: &str, value:Type){
        self.values.insert(column.to_string(), value);
    }
    pub fn get_value(&self, column: &str)->Type{
        let value = self.values.get(column);
        match value{
            Some(value) => value.clone(),
            None => panic!("No such value for {}", column),
        }
    }
    /// take the value and remove the content 
    pub fn remove<T>(&mut self, column: &str) -> T where T: FromType{
        let value = self.values.remove(column).unwrap();
        FromType::from_type(value)
    }

    /// take the value but not removing the content
    pub fn get<T>(&self, column: &str) -> T where T: FromType{
        let value = self.values.get(column).unwrap();
        FromType::from_type(value.clone())
    }
    /// get optional value
    pub fn get_opt<T>(&self, column: &str) -> Option<T> where T: FromType{
        let value = self.values.get(column);
        if value.is_some(){
            let v = value.as_ref().unwrap().clone();
            match v{
                &Type::Null => None,
                _ => Some(FromType::from_type(v.clone()))
            }
        }else{
            None
        }
    }
    
    /// get a reference of the type
    pub fn get_ref(&self, column: &str)->&Type{
        self.values.get(column).unwrap()
    }
    
    /// set the short column names using the renamed columns from the table specified
    /// will be used when casting a generic dao to multiple dao values, 
    /// ie. useful when querying 1 time using 1:1 joins
    /// TODO: deal with the optional columns
    pub fn resolve_renamed_columns(&mut self, in_table:&str, renamed_columns:&BTreeMap<String, Vec<(String, String)>>){
        let renamed_columns = renamed_columns.get(in_table);
        match renamed_columns{
            Some(renamed_columns)=>{
                for c in renamed_columns{
                    let &(ref column, ref renamed) = c;
                    println!("setting: {} to value: {}", renamed, column);
                    let orig = self.values.get(renamed).unwrap().clone();
                    self.values.insert(column.to_string(), orig);
                }
             },
            None => println!("No renamed columns for table {} on this query", in_table),
        };
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
