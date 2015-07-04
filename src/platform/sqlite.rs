use query::Query;
use dao::Dao;

use dao::Type;
use query::SqlType;
use database::{Database};
use dao::DaoResult;
use writer::SqlFrag;
use database::SqlOption;
use database::DbConfig;
use rusqlite::SqliteConnection;
use rusqlite::types::ToSql;
use rusqlite::SqliteRow;
use std::path::Path;

pub struct Sqlite {
    config: Option<DbConfig>,
    pub conn: Option<SqliteConnection>,
}


impl Sqlite{
    
    pub fn new()->Self{
        Sqlite{conn:None, config: None}
    }
    
    pub fn connect_with_url(url:&str)->Result<Self, String>{
        let path = Path::new(url);
        let conn = SqliteConnection::open(&path);
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
    
    fn from_rust_type_tosql<'a>(types: &'a Vec<Type>)->Vec<&'a ToSql>{
        let mut params:Vec<&ToSql> = vec![];
        for t in types{
            match t {
                &Type::String(ref x) => {
                    params.push(x);
                },
                _ => panic!("not yet here {:?}", t),
            };
        }
        params
    }
    
        /// convert a record of a row into rust type
    fn from_sql_to_rust_type(row: &SqliteRow, index:usize)->Type{
        let value = row.get_opt(index as i32);
         match value{
            Ok(value) => Type::String(value),
            Err(_) => Type::Null,
        }
    }
    
}

impl Database for Sqlite{
    fn get_config(&self)->DbConfig{
        self.config.clone().unwrap()
    }
    fn version(&self)->String{
        let sql = "SHOW server_version";
        let dao = self.execute_sql_with_one_return(sql, &vec![]);
        let version = dao.get("server_version");
        version
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
            SqlOption::UseNumberedParam,  // uses numbered parameters
            SqlOption::SupportsReturningClause, // supports returning clause, feature
            SqlOption::SupportsCTE,
            SqlOption::SupportsInheritance,
        ]
    }
    
    fn select(&self, query:&Query)->DaoResult{
        self.execute_with_return(query)
    }
    
    fn execute_with_return(&self, query:&Query)->DaoResult{
        let sql_frag = self.build_query(query);
        DaoResult{
            dao: self.execute_sql_with_return(&sql_frag.sql, &sql_frag.params),
            renamed_columns:query.renamed_columns.clone(),
            total:None,
            page:None,
            page_size:None,
        }
    }
    
    fn execute_with_one_return(&self, query:&Query)->Dao{
        let sql_frag = self.build_query(query);
        self.execute_sql_with_one_return(&sql_frag.sql, &sql_frag.params)
    }
    
    fn execute(&self, query:&Query)->Result<usize, String>{
        let sql_frag = self.build_query(query);
        self.execute_sql(&sql_frag.sql, &sql_frag.params)
    }
    
    fn insert(&self, query:&Query)->Dao{
        let sql_frag = self.build_insert(query);
        self.execute_sql_with_one_return(&sql_frag.sql, &sql_frag.params)
    }
    fn update(&self, query:&Query)->Dao{panic!("not yet")}
    fn delete(&self, query:&Query)->Result<usize, String>{panic!("not yet");}

    fn execute_sql_with_return(&self, sql:&str, params:&Vec<Type>)->Vec<Dao>{
        panic!("not yet");
    }

    fn execute_sql_with_return_columns(&self, sql:&str, params:&Vec<Type>, return_columns:Vec<&str>)->Vec<Dao>{
        println!("SQL: \n{}", sql);
        println!("param: {:?}", params);
        assert!(self.conn.is_some());
        let mut stmt = self.conn.as_ref().unwrap().prepare(sql).unwrap();
        let mut daos = vec![];
        let param = Self::from_rust_type_tosql(params);
        for row in stmt.query(&param).unwrap() {
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
        }
        daos
    }
    
    fn execute_sql_with_one_return(&self, sql:&str, params:&Vec<Type>)->Dao{
        let dao = self.execute_sql_with_return(sql, params);
        assert!(dao.len() == 1, "There should be 1 and only 1 record return here");
        dao[0].clone()
    }
    
    /// generic execute sql which returns not much information,
    /// returns only the number of affected records or errors
    /// can be used with DDL operations (CREATE, DELETE, ALTER, DROP)
    fn execute_sql(&self, sql:&str, params:&Vec<Type>)->Result<usize, String>{
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
    /// TODO: need to supply the number of parameters where to start the numbering of the number parameters
    fn build_query(&self, query:&Query)->SqlFrag{
        match query.sql_type{
            SqlType::SELECT => self.build_select(query),
            SqlType::INSERT => self.build_insert(query),
            SqlType::UPDATE => self.build_update(query),
            SqlType::DELETE => self.build_delete(query),
        }
    }
}