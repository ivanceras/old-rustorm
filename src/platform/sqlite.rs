use query::Query;
use dao::Dao;

use dao::Value;
use query::SqlType;
use database::{Database};
use dao::DaoResult;
use writer::SqlFrag;
use database::SqlOption;
use config::DbConfig;
use rusqlite::SqliteConnection;
use rusqlite::types::ToSql;
use rusqlite::SqliteRow;
use std::path::Path;
use table::Table;
use database::DatabaseDDL;

pub struct Sqlite {
    config: Option<DbConfig>,
    pub conn: Option<SqliteConnection>,
}


impl Sqlite{
    
    pub fn new()->Self{
        Sqlite{conn:None, config: None}
    }
    
    pub fn connect_with_url(url:&str)->Result<Self, String>{
        let config = DbConfig::from_url(url);
        let database:&str = &config.database;
        let conn = match database{
             ":memory:" => {
                println!("using in memory database");
                SqliteConnection::open_in_memory()
            },
             _ => {
                 let path = Path::new(database);
                 println!("Using file path {:?}", path);
                 println!("config{:?}", config);
                 SqliteConnection::open(&path)
             }
        };
        match conn{
            Ok(conn) => {
                let config = DbConfig::from_url(url);
                let pg = Sqlite{config: Some(config), conn: Some(conn)};
                Ok(pg)
            },
            Err(e) => {
                let error = format!("Unable to connect to database due to {}", e.message);
                println!("{:?}",e);
                Err(error)
            }
        }
    }
    
    fn from_rust_type_tosql<'a>(types: &'a Vec<Value>)->Vec<&'a ToSql>{
        let mut params:Vec<&ToSql> = vec![];
        for t in types{
            match t {
                &Value::String(ref x) => {
                    params.push(x);
                },
                _ => panic!("not yet here {:?}", t),
            };
        }
        params
    }
    
        /// convert a record of a row into rust type
    fn from_sql_to_rust_type(row: &SqliteRow, index:usize)->Value{
        let value = row.get_opt(index as i32);
         match value{
            Ok(value) => Value::String(value),
            Err(_) => Value::Null,
        }
    }
    
    ///
    /// convert rust data type names to database data type names
    /// will be used in generating SQL for table creation
    /// FIXME, need to restore the exact data type as before
    fn rust_type_to_dbtype(&self, rust_type: &str)->String{

        let rust_type = match rust_type{
            "bool" => {
                "boolean".to_string()
            },
            "i8" => {
                "integer".to_string()
            },
            "i16" => {
                "integer".to_string()
            },
            "i32"  => {
                "integer".to_string()
            },
            "u32"  => {
                "integer".to_string()
            },
            "i64"  => {
                "integer".to_string()
            },
            "f32" => {
                "real".to_string()
            },
            "f64" => {
                "real".to_string()
            },
            "String" =>{
                "text".to_string()
            },
            "Vec<u8>" =>{
                "blob".to_string()
            },
            "Json" => {
                "text".to_string()
            },
            "Uuid" => {
                "text".to_string()
            },
            "NaiveDateTime" => {
                "numeric".to_string()
            },
            "DateTime<UTC>" => {
                "numeric".to_string()
            },
            "NaiveDate" => {
                "numeric".to_string()
            },
            "NaiveTime" => {
                "numeric".to_string()
            },
            "HashMap<String, Option<String>>" => {
                "text".to_string()
            },
            _ => panic!("Unable to get the equivalent database data type for {}", rust_type),
        };
        rust_type

    }
    
}

impl Database for Sqlite{
    fn get_config(&self)->DbConfig{
        self.config.clone().unwrap()
    }
    fn version(&self)->String{
       panic!("not yet")
    }
    fn begin(&self){}
    fn commit(&self){}
    fn rollback(&self){}
    fn is_transacted(&self)->bool{false}
    fn is_closed(&self)->bool{false}
    fn is_connected(&self)->bool{false}
    fn close(&self){}
    fn is_valid(&self)->bool{false}
    fn reset(&self){}
    
    /// return this list of options, supported features in the database
    fn sql_options(&self)->Vec<SqlOption>{
        vec![
            SqlOption::UsesNumberedParam,  // uses numbered parameters
            SqlOption::SupportsCTE,
        ]
    }
    
    fn insert(&self, query:&Query)->Dao{
        let sql_frag = self.build_insert(query);
        self.execute_sql_with_one_return(&sql_frag.sql, &sql_frag.params)
    }
    fn update(&self, query:&Query)->Dao{panic!("not yet")}
    fn delete(&self, query:&Query)->Result<usize, String>{panic!("not yet");}
    
    fn execute_sql_with_return(&self, sql:&str, params:&Vec<Value>)->Vec<Dao>{
        panic!("unsupported!");
    }
    fn execute_sql_with_return_columns(&self, sql:&str, params:&Vec<Value>, return_columns:Vec<&str>)->Vec<Dao>{
        println!("SQL: \n{}", sql);
        println!("param: {:?}", params);
        assert!(self.conn.is_some());
        let mut stmt = self.conn.as_ref().unwrap().prepare(sql).unwrap();
        let mut daos = vec![];
        let param = Self::from_rust_type_tosql(params);
        let rows = stmt.query(&param);
        match rows{
            Ok(rows) => 
            for row in rows {
                match row{
                    Ok(row) => {
                        let mut index = 0;
                        let mut dao = Dao::new();
                        for rc in &return_columns{
                            let rtype = Self::from_sql_to_rust_type(&row, index);
                            dao.set_value(rc, rtype);
                            index += 1;
                        }
                        daos.push(dao);
                    }
                    Err(e) => {
                        println!("error! {}",e) 
                    }
                }
            },
            Err(e) => println!("Something is wrong")
        }
        daos
    }
    
    fn execute_sql_with_one_return(&self, sql:&str, params:&Vec<Value>)->Dao{
        let dao = self.execute_sql_with_return(sql, params);
        assert!(dao.len() == 1, "There should be 1 and only 1 record return here");
        dao[0].clone()
    }
    
    /// generic execute sql which returns not much information,
    /// returns only the number of affected records or errors
    /// can be used with DDL operations (CREATE, DELETE, ALTER, DROP)
    fn execute_sql(&self, sql:&str, params:&Vec<Value>)->Result<usize, String>{
        println!("SQL: \n{}", sql);
        println!("param: {:?}", params);
        let to_sql_types = Self::from_rust_type_tosql(params);
        assert!(self.conn.is_some());
        let result = self.conn.as_ref().unwrap().execute(sql, &to_sql_types);
        let result = match result{
            Ok(x) => { Ok(x as usize)},
            Err(e) => {
                Err(format!("Something is wrong {:?}" ,e)) 
            }
        };
        result
    }

    /// use by select to build the select query
    /// build all types of query
    fn build_query(&self, query:&Query)->SqlFrag{
        match query.sql_type{
            SqlType::SELECT => self.build_select(query),
            SqlType::INSERT => self.build_insert(query),
            SqlType::UPDATE => self.build_update(query),
            SqlType::DELETE => self.build_delete(query),
        }
    }
}

impl DatabaseDDL for Sqlite{
    fn create_schema(&self, schema:&str){
        panic!("sqlite does not support schema")
    }

    fn drop_schema(&self, schema:&str){
        panic!("sqlite does not support schema")
    }
    
    fn build_create_table(&self, table:&Table)->SqlFrag{
        let mut w = SqlFrag::new(self.sql_options());
        w.append("CREATE TABLE ");
        w.append(&table.name);
        w.append("(");
        w.ln_tab();
        let mut do_comma = false;
        for c in &table.columns{
            if do_comma {w.commasp();}else {do_comma=true;}
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
    fn create_table(&self, table:&Table){
         let frag = self.build_create_table(table);
         match self.execute_sql(&frag.sql, &vec![]){
            Ok(x) => println!("created table.."),
            Err(e) => panic!("table not created {}", e),
         }
    }

    fn rename_table(&self, table:&Table, new_tablename:String){
        
    }

    fn drop_table(&self, table:&Table){
        panic!("not yet");
    }

    fn set_foreign_constraint(&self, model:&Table){
        panic!("not yet");
    }

    fn set_primary_constraint(&self, model:&Table){
        panic!("not yet");
    }
}