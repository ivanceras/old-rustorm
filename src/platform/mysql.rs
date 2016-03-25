use query::Query;
use dao::Dao;
use table::{Table, Column, Foreign};

use dao::Value;
use writer::SqlFrag;
use database::{SqlOption, BuildMode};
use regex::Regex;

use mysql::value::Value as MyValue;
use mysql::consts::ColumnType;
use mysql::value::FromValue;
use mysql::value::IntoValue;
use mysql::error::MyResult;
use mysql::value::from_row;
use mysql::conn::Stmt;
use mysql::conn::pool::MyPool;
use chrono::naive::datetime::NaiveDateTime;
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;

use query::Operand;


use database::{Database, DatabaseDev, DatabaseDDL, DbError};
use time::Timespec;
use dao::Type;

pub struct Mysql {
    pool: Option<MyPool>,
}
impl Mysql{
    pub fn new() -> Self {
        Mysql { pool: None }
    }

    pub fn with_pooled_connection(pool: MyPool) -> Self {
        Mysql { pool: Some(pool) }
    }

    fn from_rust_type_tosql(types: &[Value]) -> Vec<MyValue> {
        let mut params: Vec<MyValue> = vec![];
        for t in types {
            match *t {
                Value::Bool(ref x) => {
                    let v = x.into_value();
                    params.push(v);
                },
                Value::String(ref x) => {
                    params.push(MyValue::Bytes(x.as_bytes().to_owned()));
                },
                Value::I8(ref x) => {
                    let v = x.into_value();
                    params.push(v);
                },
                 Value::I16(ref x) => {
                    let v = x.into_value();
                    params.push(v);
                },
                Value::I32(ref x) => {
                    let v = x.into_value();
                    params.push(v);
                },
                Value::I64(ref x) => {
                    let v = x.into_value();
                    params.push(v);
                },
                Value::U8(ref x) => {
                    let v = x.into_value();
                    params.push(v);
                },
                Value::U32(ref x) => {
                    let v = x.into_value();
                    params.push(v);
                },
                Value::U64(ref x) => {
                    let v = x.into_value();
                    params.push(v);
                },
                Value::F32(ref x) => {
                    let v = x.into_value();
                    params.push(v);
                },
                Value::F64(ref x) => {
                    let v = x.into_value();
                    params.push(v);
                },
                _ => panic!("not yet here {:?}", t),
            }
        }
        params
    }

    /// convert a record of a row into rust type
    fn from_sql_to_rust_type(row: &[MyValue], index: usize, column_type: &ColumnType) -> Value {
        let value = row.get(index);
        match value {
            Some(value) => {
                    //println!("sql to rust {:?} type: {:?}", value, column_type);
                    match *value{
                        MyValue::NULL => {
                            Value::None(Type::String)// should put Type::Unknown
                        },
                        
                        _ => {
                            match *column_type{
                                ColumnType::MYSQL_TYPE_DECIMAL => {
                                    let v: f64 = FromValue::from_value(value.clone());
                                    Value::F64(v)
                                },
                                ColumnType::MYSQL_TYPE_TINY =>{
                                    let v: i8 = FromValue::from_value(value.clone());
                                    Value::I8(v)
                                },
                                ColumnType::MYSQL_TYPE_SHORT => {
                                    let v: i16 = FromValue::from_value(value.clone());
                                    Value::I16(v)
                                },
                                ColumnType::MYSQL_TYPE_LONG => {
                                    let v: i64 = FromValue::from_value(value.clone());
                                    Value::I64(v)
                                },
                                ColumnType::MYSQL_TYPE_FLOAT =>{
                                    let v: f32 = FromValue::from_value(value.clone());
                                    Value::F32(v)
                                },
                                ColumnType::MYSQL_TYPE_DOUBLE => {
                                    let v: f64 = FromValue::from_value(value.clone());
                                    Value::F64(v)
                                },
                                ColumnType::MYSQL_TYPE_NULL => Value::None(Type::String),
                                ColumnType::MYSQL_TYPE_TIMESTAMP => {
                                    let v: Timespec = FromValue::from_value(value.clone());
                                    let t = NaiveDateTime::from_timestamp(v.sec, v.nsec as u32);
                                    debug!("time: {}",t);
                                    let t2 = DateTime::from_utc(t, UTC);
                                    Value::DateTime(t2)
                                },
                                ColumnType::MYSQL_TYPE_LONGLONG =>  {
                                    let v: i64 = FromValue::from_value(value.clone());
                                    Value::I64(v)
                                },
                                ColumnType::MYSQL_TYPE_INT24 => {
                                    let v: i32 = FromValue::from_value(value.clone());
                                    Value::I32(v)
                                },
                                ColumnType::MYSQL_TYPE_DATE => {
                                    let v: Timespec = FromValue::from_value(value.clone());
                                    let t = NaiveDateTime::from_timestamp(v.sec, v.nsec as u32);
                                    let t2 = DateTime::from_utc(t, UTC);
                                    Value::DateTime(t2)
                                },
                                ColumnType::MYSQL_TYPE_TIME => {
                                    let v: Timespec = FromValue::from_value(value.clone());
                                    let t = NaiveDateTime::from_timestamp(v.sec, v.nsec as u32);
                                    let t2 = DateTime::from_utc(t, UTC);
                                    Value::DateTime(t2)
                                },
                                ColumnType::MYSQL_TYPE_DATETIME => {
                                    let v: Timespec = FromValue::from_value(value.clone());
                                    //let t = NaiveDateTime::from_timestamp(v.sec, v.nsec as u32);
                                    //Value::NaiveDateTime(t)
                                    let t = NaiveDateTime::from_timestamp(v.sec, v.nsec as u32);
                                    let t2 = DateTime::from_utc(t, UTC);
                                    Value::DateTime(t2)
                                },
                                ColumnType::MYSQL_TYPE_YEAR => {
                                    let v: Timespec = FromValue::from_value(value.clone());
                                    let t = NaiveDateTime::from_timestamp(v.sec, v.nsec as u32);
                                    let t2 = DateTime::from_utc(t, UTC);
                                    Value::DateTime(t2)
                                },
                                ColumnType::MYSQL_TYPE_VARCHAR => {
                                    let v: String = FromValue::from_value(value.clone());
                                    Value::String(v)
                                },
                                ColumnType::MYSQL_TYPE_BIT =>unimplemented!(),
                                ColumnType::MYSQL_TYPE_NEWDECIMAL => {
                                    let v: f64 = FromValue::from_value(value.clone());
                                    Value::F64(v)
                                },
                                ColumnType::MYSQL_TYPE_ENUM => {
                                    let v: String = FromValue::from_value(value.clone());
                                    Value::String(v)
                                },
                                ColumnType::MYSQL_TYPE_SET => {
                                    let v: String = FromValue::from_value(value.clone());
                                    Value::String(v)
                                },
                                ColumnType::MYSQL_TYPE_TINY_BLOB => {
                                    let v: String = FromValue::from_value(value.clone());
                                    Value::String(v)
                                },
                                ColumnType::MYSQL_TYPE_MEDIUM_BLOB => {
                                    let v: String = FromValue::from_value(value.clone());
                                    Value::String(v)
                                },
                                ColumnType::MYSQL_TYPE_LONG_BLOB => {
                                    let v: String = FromValue::from_value(value.clone());
                                    Value::String(v)
                                },
                                ColumnType::MYSQL_TYPE_BLOB => {
                                    let v: String = FromValue::from_value(value.clone());
                                    Value::String(v)
                                },
                                ColumnType::MYSQL_TYPE_VAR_STRING => {
                                    let v: String = FromValue::from_value(value.clone());
                                    Value::String(v)
                                },
                                ColumnType::MYSQL_TYPE_STRING => {
                                    let v: String = FromValue::from_value(value.clone());
                                    Value::String(v)
                                },
                                ColumnType::MYSQL_TYPE_GEOMETRY => {
                                    let v: String = FromValue::from_value(value.clone());
                                    Value::String(v)
                                },
                            }
                        }
                    }
                    
                },
            None => Value::None(Type::String),
        }
    }

    ///
    /// convert rust data type names to database data type names
    /// will be used in generating SQL for table creation
    /// FIXME, need to restore the exact data type as before
    fn rust_type_to_dbtype(&self, rust_type: &Type) -> String {
        match *rust_type {
            Type::Bool => {
                "bool".to_owned()
            }
            Type::I8 => {
                "tinyint(1)".to_owned()
            }
            Type::I16 => {
                "integer".to_owned()
            }
            Type::I32 => {
                "integer".to_owned()
            }
            Type::U32 => {
                "integer".to_owned()
            }
            Type::I64 => {
                "integer".to_owned()
            }
            Type::F32 => {
                "real".to_owned()
            }
            Type::F64 => {
                "real".to_owned()
            }
            Type::String => {
                "text".to_owned()
            }
            Type::VecU8 => {
                "blob".to_owned()
            }
            Type::Json => {
                "text".to_owned()
            }
            Type::Uuid => {
                "varchar(36)".to_owned()
            }
            Type::NaiveDateTime => {
                "timestamp".to_owned()
            }
            Type::DateTime => {
                "timestamp".to_owned()
            }
            Type::NaiveDate => {
                "date".to_owned()
            }
            Type::NaiveTime => {
                "time".to_owned()
            }
            _ => panic!("Unable to get the equivalent database data type for {:?}",
                        rust_type),
        }
    }

	fn get_table_columns(&self, schema: &str, table: &str) -> Vec<Column> {
        let sql = format!("select column_name, data_type from information_schema.columns where table_schema='{}' and table_name='{}'", schema, table);
        assert!(self.pool.is_some());
        let mut stmt = match self.get_prepared_statement(&sql) {
            Ok(stmt) => stmt,
            Err(_) => panic!("prepare statement error.")
        };
        let mut columns = Vec::new();
        for row in stmt.execute(()).unwrap() {
            // println!("{:?}", row);
            let (name, db_data_type) = from_row::<(String, String)>(row.unwrap());
            let not_null = false;
            // let name: String = row.get("column_name");
            // let not_null: bool = row.get("is_nullable");
            // let db_data_type: String = row.get("data_type");
            //TODO: temporarily regex the data type to extract the size as well
            let re = match Regex::new("(.+)\\((.+)\\)") {
                Ok(re) => re,
                Err(err) => panic!("{}", err),
            };

            let db_data_type = if re.is_match(&db_data_type) {
                let cap = re.captures(&db_data_type).unwrap();
                let data_type = cap.at(1).unwrap().to_owned();
                // TODO::can be use in the later future
                // let size = cap.at(2).unwrap().to_owned();
                data_type
            } else {
                db_data_type
            };

            // let is_primary: bool = row.get("is_primary");
            // let is_unique: bool = row.get("is_unique");
            let is_primary: bool = false;
            let is_unique: bool = false;

            // let default: Option<Operand> = match row.get_opt("default") {
            //     Ok(x) => Some(Operand::Value(Value::String(x))),
            //     Err(_) => None,
            // };
            let default: Option<Operand> = None;
            // let comment: Option<String> = match row.get_opt("comment") {
            //     Ok(x) => Some(x),
            //     Err(_) => None,
            // };
            let comment: Option<String> = None;

            // let foreign_schema: Option<String> = match row.get_opt("foreign_schema") {
            //     Ok(x) => Some(x),
            //     Err(_) => None,
            // };
            let foreign_schema: Option<String> = None;
            
            // let foreign_column: Option<String> = match row.get_opt("foreign_column") {
            //     Ok(x) => Some(x),
            //     Err(_) => None,
            // };
            let foreign_column: Option<String> = None;
            
            // let foreign_table: Option<String> = match row.get_opt("foreign_table") {
            //     Ok(x) => Some(x),
            //     Err(_) => None,
            // };
            let foreign_table: Option<String> = None;

            let foreign = if foreign_table.is_some() && foreign_column.is_some() &&
                             foreign_schema.is_some() {
                Some(Foreign {
                    schema: foreign_schema,
                    table: foreign_table.unwrap(),
                    column: foreign_column.unwrap(),
                })

            } else {
                None
            };
            let (_, data_type) = self.dbtype_to_rust_type(&db_data_type);
            let column = Column {
                table: Some(table.to_owned()),
                name: name,
                data_type: data_type,
                db_data_type: db_data_type,
                comment: comment,
                is_primary: is_primary,
                is_unique: is_unique,
                default: default,
                not_null: not_null,
                foreign: foreign,
                is_inherited: false, /* will be corrected later in the get_meta_data */
            };
            columns.push(column);
        }
        
        columns
        
    }

    fn get_table_comment(&self, schema: &str, table: &str) -> Option<String> {
        
        None
    }


    fn get_prepared_statement<'a>(&'a self, sql: &'a str) -> MyResult<Stmt> {
        self.pool.as_ref().unwrap().prepare(sql)
    }
}

impl Database for Mysql {
    fn version(&self) -> Result<String, DbError> {
        let sql = "SELECT version()";
        let dao = try!(self.execute_sql_with_one_return(sql, &vec![]));
        match dao {
            Some(dao) => Ok(dao.get("version()")),
            None => Err(DbError::new("Unable to get database version")),
        }
    }

    fn begin(&self) {
        unimplemented!()
    }
    fn commit(&self) {
        unimplemented!()
    }
    fn rollback(&self) {
        unimplemented!()
    }
    fn is_transacted(&self) -> bool {
        false
    }
    fn is_closed(&self) -> bool {
        false
    }
    fn is_connected(&self) -> bool {
        false
    }
    fn close(&self) {
    }
    fn is_valid(&self) -> bool {
        false
    }
    fn reset(&self) {
        unimplemented!()
    }

    /// return this list of options, supported features in the database
    fn sql_options(&self) -> Vec<SqlOption> {
        vec![
            SqlOption::UsesQuestionMark,//mysql uses question mark instead of the numbered params
        ]
    }

    fn update(&self, _query: &Query) -> Dao {
        unimplemented!()
    }
    fn delete(&self, _query: &Query) -> Result<usize, String> {
        unimplemented!()
    }

    fn execute_sql_with_return(&self, sql: &str, params: &[Value]) -> Result<Vec<Dao>, DbError> {
        debug!("SQL: \n{}", sql);
        debug!("param: {:?}", params);
        assert!(self.pool.is_some());
        let mut stmt = try!(self.get_prepared_statement(sql));
        let mut columns = vec![];
        for col in stmt.columns_ref().unwrap() {
            let column_name = String::from_utf8(col.name.clone()).unwrap();
            debug!("column type: {:?}", col.column_type);
            columns.push( (column_name, col.column_type) );
        }
        let mut daos = vec![];
        let param = Mysql::from_rust_type_tosql(params);
        let rows = try!(stmt.execute(&param));
        for row in rows {
            let row = try!(row);
            let mut index = 0;
            let mut dao = Dao::new();
            for &(ref column_name, ref column_type) in &columns {
                let rtype = Mysql::from_sql_to_rust_type(&row, index, column_type);
                dao.set_value(&column_name, rtype);
                index += 1;
            }
            daos.push(dao);
        }
        Ok(daos)
    }

    fn execute_sql_with_one_return(&self,
                                   sql: &str,
                                   params: &[Value])
                                   -> Result<Option<Dao>, DbError> {
        let dao = try!(self.execute_sql_with_return(sql, params));
        if dao.len() >= 1 {
            Ok(Some(dao[0].clone()))
        } else {
            Ok(None)
        }
    }

    /// generic execute sql which returns not much information,
    /// returns only the number of affected records or errors
    /// can be used with DDL operations (CREATE, DELETE, ALTER, DROP)
    fn execute_sql(&self, sql: &str, params: &[Value]) -> Result<usize, DbError> {
        debug!("SQL: \n{}", sql);
        debug!("param: {:?}", params);
        let to_sql_types = Mysql::from_rust_type_tosql(params);
        assert!(self.pool.is_some());
        let result = try!(self.pool.as_ref().unwrap().prep_exec(sql, &to_sql_types));
        Ok(result.affected_rows() as usize)
    }

}

impl DatabaseDDL for Mysql{
    fn create_schema(&self, _schema: &str) {
        unimplemented!()
    }

    fn drop_schema(&self, _schema: &str) {
        unimplemented!()
    }

    fn build_create_table(&self, table: &Table) -> SqlFrag {
        let mut w = SqlFrag::new(self.sql_options(), BuildMode::Standard);
        w.append("CREATE TABLE ");
        w.append(&table.name);
        w.append("(");
        w.ln_tab();
        let mut do_comma = false;
        for c in &table.columns {
            if do_comma {
                w.commasp();
            } else {
                do_comma = true;
            }
            w.append(&c.name);
            w.append(" ");
            let dt = self.rust_type_to_dbtype(&c.data_type);
            w.append(&dt);
            if c.is_primary {
                w.append(" PRIMARY KEY ");
            }
        }
        w.append(")");
        w
    }
    fn create_table(&self, table: &Table) {
        let frag = self.build_create_table(table);
        match self.execute_sql(&frag.sql, &vec![]) {
            Ok(_) => debug!("created table.."),
            Err(e) => panic!("table not created {}", e),
        }
    }

    fn rename_table(&self, _table: &Table, _new_tablename: String) {
        unimplemented!()
    }

    fn drop_table(&self, _table: &Table) {
        unimplemented!()
    }

    fn set_foreign_constraint(&self, _model: &Table) {
        unimplemented!()
    }

    fn set_primary_constraint(&self, _model: &Table) {
        unimplemented!()
    }
}


// TODO: need to implement trait DatabaseDev for Mysql
// Mysql can be used as development database




    
impl DatabaseDev for Mysql {

    fn get_parent_table(&self, schema: &str, table: &str) -> Option<String> {
        None
    }

    fn get_table_sub_class(&self, schema: &str, table: &str) -> Vec<String> {
        vec![]
    }

    fn get_table_metadata(&self, schema: &str, table: &str, is_view: bool) -> Table {

        let mut columns = self.get_table_columns(schema, table);
        let comment = self.get_table_comment(schema, table);
        let parent = self.get_parent_table(schema, table);
        let subclass = self.get_table_sub_class(schema, table);

        //mutate columns to mark those which are inherited
        if parent.is_some() {
            let inherited_columns = self.get_inherited_columns(schema, table);
            for i in inherited_columns {
                for c in &mut columns {
                    if i == c.name {
                        c.is_inherited = true;
                    }
                }
            }
        }

        Table {
            schema: Some(schema.to_owned()),
            name: table.to_owned(),
            parent_table: parent,
            sub_table: subclass,
            comment: comment,
            columns: columns,
            is_view: is_view,
        }
    }

    fn get_all_tables(&self) -> Vec<(String, String, bool)> {
        let sql = "SELECT schema()";
        let schema_name: String = match self.execute_sql_with_one_return(sql, &vec![]) {
            Ok(dao) => {
                match dao {
                    Some(dao) => dao.get("schema()"),
                    None => panic!("Unable to get current schema.")
                }
            },
            Err(_) => panic!("can not get current schema.")
        };
        
        let sql = format!("select table_name from information_schema.tables where table_schema='{}' and table_type='base table'", schema_name);
        assert!(self.pool.is_some());
        let mut stmt = match self.get_prepared_statement(&sql) {
            Ok(stmt) => stmt,
            Err(_) => panic!("prepare statement error.")
        };
        let mut tables: Vec<(String, String, bool)> = Vec::new();
        for row in stmt.execute(()).unwrap() {
            let (table_name,) = from_row::<(String,)>(row.unwrap());
            tables.push((schema_name.clone(), table_name, false));
        }
        tables
    }



    fn get_inherited_columns(&self, schema: &str, table: &str) -> Vec<String> {
        vec![]
    }


    /// get the rust data type names from database data type names
    /// will be used in source code generation
    fn dbtype_to_rust_type(&self, db_type: &str) -> (Vec<String>, Type) {
        match db_type {
            "bool" => {
                (vec![], Type::Bool)
            }
            "tinyint" => {
                (vec![], Type::I8)
            }
            "smallint"  => {
                (vec![], Type::I16)
            }
            "integer" | "int" => {
                (vec![], Type::I32)
            }
            "bigint" => {
                (vec![], Type::I64)
            }
            "float" => {
                (vec![], Type::F32)
            }
            "double" | "decimal" => {
                (vec![], Type::F64)
            }
            "char" | "varchar" | "text" => {
                (vec![], Type::String)
            }
            "blob" => {
                (vec![], Type::VecU8)
            }
            "timestamp" | "datetime" => {
                (vec!["chrono::datetime::DateTime".to_owned(),
                      "chrono::offset::utc::UTC".to_owned()],
                 Type::DateTime)
                //(vec!["chrono::naive::date::NaiveDateTime".to_owned()],
                // Type::NaiveDateTime)
            }
            "date" => {
                (vec!["chrono::naive::date::NaiveDate".to_owned()],
                 Type::NaiveDate)
            }
            "time" => {
                (vec!["chrono::naive::time::NaiveTime".to_owned()],
                 Type::NaiveTime)
            }
            _ => panic!("Unable to get the equivalent data type for {}", db_type),
        }
    }

    ///
    /// convert rust data type names to database data type names
    /// will be used in generating SQL for table creation
    /// FIXME, need to restore the exact data type as before
    fn rust_type_to_dbtype(&self, rust_type: &Type) -> String {
        match *rust_type {
            Type::Bool => {
                "bool".to_owned()
            }
            Type::I8 => {
                "tinyint(1)".to_owned()
            }
            Type::I16 => {
                "integer".to_owned()
            }
            Type::I32 => {
                "integer".to_owned()
            }
            Type::U32 => {
                "integer".to_owned()
            }
            Type::I64 => {
                "integer".to_owned()
            }
            Type::F32 => {
                "real".to_owned()
            }
            Type::F64 => {
                "real".to_owned()
            }
            Type::String => {
                "text".to_owned()
            }
            Type::VecU8 => {
                "blob".to_owned()
            }
            Type::Json => {
                "text".to_owned()
            }
            Type::Uuid => {
                "varchar(36)".to_owned()
            }
            Type::NaiveDateTime => {
                "timestamp".to_owned()
            }
            Type::DateTime => {
                "timestamp".to_owned()
            }
            Type::NaiveDate => {
                "date".to_owned()
            }
            Type::NaiveTime => {
                "time".to_owned()
            }
            _ => panic!("Unable to get the equivalent database data type for {:?}",
                        rust_type),
        }
    }

}
