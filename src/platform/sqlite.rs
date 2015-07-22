use query::Query;
use dao::Dao;

use dao::Value;
use database::{Database};
use writer::SqlFrag;
use database::SqlOption;
use rusqlite::SqliteConnection;
use rusqlite::types::ToSql;
use rusqlite::SqliteRow;
use table::Table;
use database::DatabaseDDL;
use database::DbError;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

pub struct Sqlite {
    pool: Option<PooledConnection<SqliteConnectionManager>>,
}


impl Sqlite{
    
    pub fn new()->Self{
        Sqlite{pool: None}
    }
    
    pub fn with_pooled_connection(pool: PooledConnection<SqliteConnectionManager>)->Self{
       Sqlite{pool: Some(pool)}
    }
    
    fn from_rust_type_tosql<'a>(&self, types: &'a Vec<Value>)->Vec<&'a ToSql>{
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
    
    pub fn get_connection(&self)->&SqliteConnection{
       if self.pool.is_some(){
            &self.pool.as_ref().unwrap()
        }
        else{
            panic!("No connection for this database")
        }
    }
    
        /// convert a record of a row into rust type
    fn from_sql_to_rust_type(&self, row: &SqliteRow, index:usize)->Value{
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
    fn version(&mut self)->String{
       let sql = "select sqlite_version() as version";
       let dao = self.execute_sql_with_return_columns(sql, &vec![], vec!["version"]);
       match dao{
            Ok(dao) => {
                if dao.len() == 1{
                    return dao[0].get("version")
                }
                else{
                    return "unknown".to_string()
                }
            },
            Err(_) => panic!("unable to get database version")
        }
    }
    fn begin(&mut self){}
    fn commit(&mut self){}
    fn rollback(&mut self){}
    fn is_transacted(&mut self)->bool{false}
    fn is_closed(&mut self)->bool{false}
    fn is_connected(&mut self)->bool{false}
    fn close(&mut self){}
    fn is_valid(&mut self)->bool{false}
    fn reset(&mut self){}
    
    /// return this list of options, supported features in the database
    fn sql_options(&self)->Vec<SqlOption>{
        vec![
            SqlOption::UsesNumberedParam,  // uses numbered parameters
            SqlOption::SupportsCTE,
        ]
    }
    
    fn insert(&mut self, query:&Query)->Result<Dao, DbError>{
        let sql_frag = self.build_insert(query);
        self.execute_sql_with_one_return(&sql_frag.sql, &sql_frag.params)
    }
    fn update(&mut self, query:&Query)->Dao{panic!("not yet")}
    fn delete(&mut self, query:&Query)->Result<usize, String>{panic!("not yet");}
    
    /// sqlite does not return the columns mentioned in the query,
    /// you have to specify it yourself
    fn execute_sql_with_return(&mut self, sql:&str, params:&Vec<Value>)->Result<Vec<Dao>, DbError>{
        panic!("unsupported!");
    }
    fn execute_sql_with_return_columns(&mut self, sql:&str, params:&Vec<Value>, return_columns:Vec<&str>)->Result<Vec<Dao>, DbError>{
        println!("SQL: \n{}", sql);
        println!("param: {:?}", params);
        let conn = self.get_connection();
        let mut stmt = conn.prepare(sql).unwrap();
        let mut daos = vec![];
        let param = self.from_rust_type_tosql(params);
        match stmt.query(&param){
            Ok(rows) => 
            for row in rows {
                match row{
                    Ok(row) => {
                        let mut index = 0;
                        let mut dao = Dao::new();
                        for rc in &return_columns{
                            let rtype = self.from_sql_to_rust_type(&row, index);
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
        Ok(daos)
    }
    
    fn execute_sql_with_one_return(&mut self, sql:&str, params:&Vec<Value>)->Result<Dao, DbError>{
        let dao = self.execute_sql_with_return(sql, params);
        match dao{
            Ok(dao) => {
                if dao.len() == 1{
                    return Ok(dao[0].clone())
                }
                else{
                    return Err(DbError::new("There should be 1 and only 1 record return here"))
                }
            },
            Err(e) => Err(DbError::new("Error in the query"))
        }
    }
    
    /// generic execute sql which returns not much information,
    /// returns only the number of affected records or errors
    /// can be used with DDL operations (CREATE, DELETE, ALTER, DROP)
    fn execute_sql(&mut self, sql:&str, params:&Vec<Value>)->Result<usize, DbError>{
        println!("SQL: \n{}", sql);
        println!("param: {:?}", params);
        let to_sql_types = self.from_rust_type_tosql(params);
        let conn = self.get_connection();
        let result = conn.execute(sql, &to_sql_types);
        match result{
            Ok(result) => { Ok(result as usize)},
            Err(e) => {
                Err(DbError::new("Something is wrong")) 
            }
        }
    }

}

impl DatabaseDDL for Sqlite{
    fn create_schema(&mut self, schema:&str){
        panic!("sqlite does not support schema")
    }

    fn drop_schema(&mut self, schema:&str){
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
    fn create_table(&mut self, table:&Table){
         let frag = self.build_create_table(table);
         match self.execute_sql(&frag.sql, &vec![]){
            Ok(x) => println!("created table.."),
            Err(e) => panic!("table not created {}", e),
         }
    }

    fn rename_table(&mut self, table:&Table, new_tablename:String){
        
    }

    fn drop_table(&mut self, table:&Table){
        panic!("not yet");
    }

    fn set_foreign_constraint(&mut self, model:&Table){
        panic!("not yet");
    }

    fn set_primary_constraint(&mut self, model:&Table){
        panic!("not yet");
    }
}