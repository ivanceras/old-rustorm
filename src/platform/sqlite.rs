use query::Query;
use dao::Dao;

use dao::Value;
use database::{Database,DatabaseDev};
use writer::SqlFrag;
use database::SqlOption;
use rusqlite::SqliteConnection;
use rusqlite::types::ToSql;
use rusqlite::SqliteRow;
use table::{Table, Column, Foreign};
use database::DatabaseDDL;
use database::DbError;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use regex::Regex;
use std::collections::BTreeMap;

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
    
    /// get the foreign keys of table
    fn get_foreign_keys(&self, schema:&str, table:&str)->Vec<Foreign>{
        println!("Extracting foreign keys...");
        let sql = format!("PRAGMA foreign_key_list({});", table);
        let result = self.execute_sql_with_return(&sql, &vec![]);
        println!("result: {:#?}", result);
        match result{
            Ok(result) => {
                let mut foreigns = vec![];
                for r in result{
                    let table: String = r.get("table");
                    let from: String = r.get("from");
                    let to: String = r.get("to");
                    println!("table: {}", table);
                    println!("from: {}", from);
                    println!("to: {}", to);
                    
                    let foreign = Foreign{
                        schema: "".to_string(),
                        table: table.to_string(),
                        column: to.to_string(),
                    };
                    foreigns.push(foreign);
                }
                foreigns
            },
            Err(e) => {
                println!("Something is wrong {}", e);
                vec![]
            }
        }
    }
    
    pub fn extract_comments(create_sql: &str)->
                    Result<(Option<String>, BTreeMap<String, Option<String>>),DbError>{
        let re = match Regex::new(r".*CREATE\s+TABLE\s+(\S+)\s*\((?s)(.*)\).*") {
            Ok(re) => re,
            Err(err) => panic!("{}", err),
        };
        println!("create_sql: {:?}", create_sql);
        if re.is_match(&create_sql){
            println!("matched...");
            let cap = re.captures(&create_sql).unwrap();
            let all_columns = cap.at(2).unwrap();
            
            let line_comma_re = match Regex::new(r"[,\n]") {
                Ok(re) => re,
                Err(err) => panic!("{}", err),
            };
            println!("All columns.. {}", all_columns);
            let splinters:Vec<&str> = line_comma_re.split(all_columns).collect();
            println!("splinters: {:#?}", splinters);
            let splinters:Vec<&str>  = splinters.into_iter()
                .map(|i|i.trim())
                .filter(|&i|i !="").collect();
            println!("filtered: {:#?}",splinters);
            let mut columns: Vec<String> = vec![];
            let mut comments: Vec<Option<String>> = vec![];
            let mut index = 0;
            for splinter in splinters{
                if splinter.starts_with("--"){
                    if comments.len() < index {
                        for i in comments.len()..index{
                            comments.push(None);
                        }
                    }
                    comments.push(Some(splinter.to_string()));
                }
                else if splinter.starts_with("FOREIGN"){
                
                }
                else if splinter.starts_with("CHECK"){
                
                }
                else{
                    let line: Vec<&str> = splinter.split_whitespace().collect();
                    let column = line[0];
                    println!("column: {}", column);
                    columns.push(column.to_string());
                    index += 1
                }
            }
            println!("columns: {:#?}",columns);
            println!("comments: {:#?}", comments);
            let table_comment = if comments.len() > 0{
                 comments[0].clone() //first comment is the table comment
            }else{None};
            let mut column_comments = BTreeMap::new();
            let mut index = 0;
            for column in columns{
                let comment = if comments.len() > 0 {comments[index + 1].clone()}else{None};
                column_comments.insert(column, comment);
                index += 1;
            }
            Ok((table_comment, column_comments))
        }
        else{
            Err(DbError::new("Unable to parse sql statement"))
        }
    }
    /// extract the comment of the table
    /// Don't support multi-line comment
    fn get_table_comment(&self, schema:&str, table:&str)->Option<String>{   
        let sql = format!("SELECT sql FROM sqlite_master WHERE type = 'table' AND tbl_name = '{}'",table);
        let result = self.execute_sql_with_return(&sql, &vec![]);
        match result{
            Ok(result) => {
                assert_eq!(result.len(), 1);
                let ref dao = result[0];
                let create_sql:String = dao.get("sql");
                match Sqlite::extract_comments(&create_sql){
                    Ok((table_comment, column_comments)) => {
                        println!("table_comment: {:?}", table_comment);
                        table_comment
                    },
                    Err(e) => {
                        None
                    }
                }
            },
            Err(e) => None
        }
    }
    /// extract the comments for each column
    /// Don't support multi-line comment
    fn get_column_comments(&self, schema:&str, table:&str)->BTreeMap<String, Option<String>>{   
        let sql = format!("SELECT sql FROM sqlite_master WHERE type = 'table' AND tbl_name = '{}'",table);
        let result = self.execute_sql_with_return(&sql, &vec![]);
        match result{
            Ok(result) => {
                assert_eq!(result.len(), 1);
                let ref dao = result[0];
                let create_sql:String = dao.get("sql");
                match Sqlite::extract_comments(&create_sql){
                    Ok((table_comment, column_comments)) => {
                        println!("column_comments: {:?}", column_comments);
                        column_comments
                    },
                    Err(e) => {
                        BTreeMap::new()
                    }
                }
            },
            Err(e) => BTreeMap::new()
        }
    }
    
    fn get_column_comment(&self, column_comments: &BTreeMap<String, Option<String>>, column: &str)->Option<String>{
        match column_comments.get(column){
            Some(comment) => comment.clone(),
            None => None
        }
    
    }
    fn get_column_foreign(&self, all_foreign: &Vec<Foreign>, column: &str)->Option<Foreign>{
        println!("foreign: {:#?} ", all_foreign);
        for foreign in all_foreign{
            if foreign.column == column{
                return Some(foreign.clone())
            }
        }
        None
    }
}

impl Database for Sqlite{
    fn version(&self)->String{
       let sql = "select sqlite_version() as version";
       let dao = self.execute_sql_with_return(sql, &vec![]);
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
    
    fn insert(&self, query:&Query)->Result<Dao, DbError>{
        let sql_frag = self.build_insert(query);
        self.execute_sql_with_one_return(&sql_frag.sql, &sql_frag.params)
    }
    fn update(&self, query:&Query)->Dao{panic!("not yet")}
    fn delete(&self, query:&Query)->Result<usize, String>{panic!("not yet");}
    
    /// sqlite does not return the columns mentioned in the query,
    /// you have to specify it yourself
    /// TODO: found this 
    /// http://jgallagher.github.io/rusqlite/rusqlite/struct.SqliteStatement.html#method.column_names
    fn execute_sql_with_return(&self, sql:&str, params:&Vec<Value>)->Result<Vec<Dao>, DbError>{
        println!("SQL: \n{}", sql);
        println!("param: {:?}", params);
        let conn = self.get_connection();
        let mut stmt = conn.prepare(sql).unwrap();
        let mut daos = vec![];
        let param = self.from_rust_type_tosql(params);
        let mut columns = vec![];
        for c in stmt.column_names(){
            columns.push(c.to_string());
        }
        println!("columns : {:?}", columns);
        match stmt.query(&param){
            Ok(rows) => 
            for row in rows {
                match row{
                    Ok(row) => {
                        let mut index = 0;
                        let mut dao = Dao::new();
                        for col in &columns{
                            let rtype = self.from_sql_to_rust_type(&row, index);
                            println!("{:?}",rtype);
                            dao.set_value(col, rtype);
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

    
    fn execute_sql_with_one_return(&self, sql:&str, params:&Vec<Value>)->Result<Dao, DbError>{
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
    fn execute_sql(&self, sql:&str, params:&Vec<Value>)->Result<usize, DbError>{
        println!("SQL: \n{}", sql);
        println!("param: {:?}", params);
        let to_sql_types = self.from_rust_type_tosql(params);
        let conn = self.get_connection();
        let result = conn.execute(sql, &to_sql_types);
        match result{
            Ok(result) => { Ok(result as usize)},
            Err(e) => {
                Err(DbError::new(&format!("Something is wrong, {}", e))) 
            }
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
        
        fn build_foreign_key_stmt(table: &Table)->SqlFrag{
            let mut w = SqlFrag::new(vec![]);
            let mut do_comma = true;//there has been colcommentsumns mentioned
            for c in &table.columns{
                if c.foreign.is_some(){
                    if do_comma {w.commasp();}else {do_comma=true;}
                    let foreign = c.foreign.as_ref().unwrap();
                    w.ln_tab();
                    w.append("FOREIGN KEY");
                    w.append(&format!("({})",c.name));
                    w.append(" REFERENCES ");
                    w.append(&format!("{}", foreign.table));
                    w.append(&format!("({})",foreign.column));
                }
            }
            w
        }
        
        let mut w = SqlFrag::new(self.sql_options());
        w.append("CREATE TABLE ");
        w.append(&table.name);
        w.append("(");
        w.ln_tab();
        let mut do_comma = false;
        for c in &table.columns{
            if do_comma {w.commasp();w.ln_tab();}else {do_comma=true;}
            w.append(&c.name);
            w.append(" ");
            let dt = self.rust_type_to_dbtype(&c.data_type);
            w.append(&dt);
            if c.is_primary {
                w.append(" PRIMARY KEY ");
            }
        }
        let fsql = build_foreign_key_stmt(table);
        w.append(&fsql.sql);
        w.ln();
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

impl DatabaseDev for Sqlite{
    fn get_table_sub_class(&self, schema:&str, table:&str)->Vec<String>{panic!("not yet")}

    fn get_parent_table(&self, schema:&str, table:&str)->Option<String>{panic!("not yet")}
    
    fn get_table_metadata(&self, schema:&str, table:&str, is_view: bool)->Table{
        println!("extracting table meta data in sqlite");
        let sql = format!("PRAGMA table_info({});", table);
        let result = self.execute_sql_with_return(&sql, &vec![]);
        println!("result: {:#?}", result);
        match result{
            Ok(result) => {
                let foreign = self.get_foreign_keys(schema, table);
                let table_comment = self.get_table_comment(schema, table);
                let column_comments = self.get_column_comments(schema, table);
                
                let mut columns = vec![];
                for r in result{
                    let column: String = r.get("name");
                    let data_type: String = r.get("type");
                    let default_value: String = r.get("dflt_value");
                    let not_null: String = r.get("notnull");
                    let pk: String = r.get("pk");
                    println!("column: {}", column);
                    println!("data_type: {}", data_type);
                    println!("not null: {}", not_null);
                    println!("pk: {}", pk);
                    println!("default_value: {}", default_value);
                    
                    let column_comment = self.get_column_comment(&column_comments, &column);
                    let column_foreign = self.get_column_foreign(&foreign, &column);
                    let column = Column{
                            name: column,
                            data_type: data_type.to_string(),
                            db_data_type: data_type.to_string(),
                            is_primary: pk != "0",
                            is_unique: false,
                            default: Some(default_value),
                            comment: column_comment,
                            not_null : not_null != "0",
                            is_inherited: false,
                            foreign: column_foreign,
                        };
                    columns.push(column);
                }
                Table{
                    schema: "".to_string(),
                    name: table.to_string(),
                    parent_table: None,
                    sub_table: vec![],
                    comment: table_comment,
                    columns: columns,
                    is_view: false,
                }
            },
            Err(e) => {
                panic!("No matching table found {}", e);
            }
        }
    }

    fn get_all_tables(&self)->Vec<(String, String, bool)>{
        let sql = "SELECT type, name, tbl_name, sql FROM sqlite_master WHERE type = 'table'";
        let result = self.execute_sql_with_return(&sql, &vec![]);
        match result{
            Ok(result) => {
                let mut tables:Vec<(String, String, bool)> = Vec::new();
                for r in result{
                    let schema = "".to_string();
                    let table: String = r.get("tbl_name");
                    let is_view = false;
                    tables.push((schema, table, is_view))
                }
                tables 
            },
            Err(e) => {
                panic!("Unable to get tables due to {}", e)
            }
        }
    }

    fn get_inherited_columns(&self, schema:&str, table:&str)->Vec<String>{
        vec![]
    }

    fn dbtype_to_rust_type(&self, db_type: &str)->(Vec<String>, String){panic!("not yet")}
    
    fn rust_type_to_dbtype(&self, rust_type: &str)->String{panic!("not yet")}
}


#[test]
fn test_comment_extract(){
    let create_sql = r"
CREATE TABLE product_availability ( 
   --Each product has its own product availability which determines when can it be available for purchase
    product_id uuid NOT NULL , --this is the id of the product
    available boolean,
    always_available boolean, 
    stocks numeric DEFAULT 1, 
    available_from timestamp with time zone,
    available_until timestamp with time zone,
    available_day json, 
    open_time time with time zone, 
    close_time time with time zone, --closing time
    FOREIGN KEY(product_id) REFERENCES product(product_id)
)    
    ";
    Sqlite::extract_comments(create_sql);
}