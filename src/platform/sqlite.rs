use query::Query;
use dao::Dao;

use dao::Value;
use database::{Database, DatabaseDev, BuildMode};
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
use dao::Type;
use query::Operand;

pub struct Sqlite {
    pool: Option<PooledConnection<SqliteConnectionManager>>,
}

impl Sqlite {
    pub fn new() -> Self {
        Sqlite { pool: None }
    }

    pub fn with_pooled_connection(pool: PooledConnection<SqliteConnectionManager>) -> Self {
        Sqlite { pool: Some(pool) }
    }

    fn from_rust_type_tosql<'a>(&self, types: &'a [Value]) -> Vec<&'a ToSql> {
        let mut params: Vec<&ToSql> = vec![];
        for t in types {
            match t {
                &Value::String(ref x) => {
                    params.push(x);
                }
                _ => panic!("not yet here {:?}", t),
            }
        }
        params
    }

    pub fn get_connection(&self) -> &SqliteConnection {
        match self.pool {
            Some(ref pool) => &pool,
            None => panic!("No connection for this database")
        }
    }

    /// convert a record of a row into rust type
    fn from_sql_to_rust_type(&self, row: &SqliteRow, index: usize) -> Value {
        let value = row.get_checked(index as i32);
        match value {
            Ok(value) => Value::String(value),
            Err(_) => Value::None(Type::String),
        }
    }

    ///
    /// convert rust data type names to database data type names
    /// will be used in generating SQL for table creation
    /// FIXME, need to restore the exact data type as before
    fn rust_type_to_dbtype(&self, rust_type: &Type) -> String {

        let rust_type = match *rust_type {
            Type::Bool => {
                "boolean".to_owned()
            }
           Type::I8 => {
                "integer".to_owned()
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
                "text".to_owned()
            }
            Type::NaiveDateTime => {
                "numeric".to_owned()
            }
            Type::DateTime => {
                "numeric".to_owned()
            }
            Type::NaiveDate => {
                "numeric".to_owned()
            }
            Type::NaiveTime => {
                "numeric".to_owned()
            }
            _ => panic!("Unable to get the equivalent database data type for {:?}",
                        rust_type),
        };
        rust_type

    }

    /// get the foreign keys of table
    fn get_foreign_keys(&self, _schema: &str, table: &str) -> Vec<Foreign> {
        debug!("Extracting foreign keys...");
        let sql = format!("PRAGMA foreign_key_list({});", table);
        let result = self.execute_sql_with_return(&sql, &vec![]).unwrap();
        debug!("result: {:#?}", result);
        let mut foreigns = vec![];
        for r in result {
            let table: String = r.get("table");
            let from: String = r.get("from");
            let to: String = r.get("to");
            debug!("table: {}", table);
            debug!("from: {}", from);
            debug!("to: {}", to);

            let foreign = Foreign {
                schema: None,
                table: table.to_owned(),
                column: to.to_owned(),
            };
            foreigns.push(foreign);
        }
        foreigns
    }

    pub fn extract_comments
                            (create_sql: &str)
                             -> Result<(Option<String>, BTreeMap<String, Option<String>>), DbError> {
        let re = try!(Regex::new(r".*CREATE\s+TABLE\s+(\S+)\s*\((?s)(.*)\).*"));
        debug!("create_sql: {:?}", create_sql);
        if re.is_match(&create_sql) {
            debug!("matched...");
            let cap = re.captures(&create_sql).unwrap();
            let all_columns = cap.at(2).unwrap();

            let line_comma_re = try!(Regex::new(r"[,\n]"));
            debug!("All columns.. {}", all_columns);
            let splinters: Vec<&str> = line_comma_re.split(all_columns).collect();
            debug!("splinters: {:#?}", splinters);
            let splinters: Vec<&str> = splinters.into_iter()
                                                .map(|i| i.trim())
                                                .filter(|&i| i != "")
                                                .collect();
            debug!("filtered: {:#?}", splinters);
            let mut columns: Vec<String> = vec![];
            let mut comments: Vec<Option<String>> = vec![];
            let mut index = 0;
            for splinter in splinters {
                if splinter.starts_with("--") {
                    if comments.len() < index {
                        for _ in comments.len()..index {
                            comments.push(None);
                        }
                    }
                    comments.push(Some(splinter.to_owned()));
                } else if splinter.starts_with("FOREIGN") {

                } else if splinter.starts_with("CHECK") {

                } else {
                    let line: Vec<&str> = splinter.split_whitespace().collect();
                    let column = line[0];
                    debug!("column: {}", column);
                    columns.push(column.to_owned());
                    index += 1
                }
            }
            debug!("columns: {:#?}", columns);
            debug!("comments: {:#?}", comments);
            let table_comment = if comments.len() > 0 {
                comments[0].clone() //first comment is the table comment
            } else {
                None
            };
            let mut column_comments = BTreeMap::new();
            let mut index = 0;
            for column in columns {
                let comment = if comments.len() > 0 {
                    comments[index + 1].clone()
                } else {
                    None
                };
                column_comments.insert(column, comment);
                index += 1;
            }
            Ok((table_comment, column_comments))
        } else {
            Err(DbError::new("Unable to parse sql statement"))
        }
    }
    /// extract the comment of the table
    /// Don't support multi-line comment
    fn get_table_comment(&self, _schema: &str, table: &str) -> Option<String> {
        let sql = format!("SELECT sql FROM sqlite_master WHERE type = 'table' AND tbl_name = '{}'",
                          table);
        let result = self.execute_sql_with_return(&sql, &vec![]).unwrap();
        assert_eq!(result.len(), 1);
        let ref dao = result[0];
        let create_sql: String = dao.get("sql");
        match Sqlite::extract_comments(&create_sql) {
            Ok((table_comment, _column_comments)) => {
                debug!("table_comment: {:?}", table_comment);
                table_comment
            }
            Err(_) => {
                None
            }
        }
    }
    /// extract the comments for each column
    /// Don't support multi-line comment
    fn get_column_comments(&self, _schema: &str, table: &str) -> BTreeMap<String, Option<String>> {
        let sql = format!("SELECT sql FROM sqlite_master WHERE type = 'table' AND tbl_name = '{}'",
                          table);
        let result = self.execute_sql_with_return(&sql, &vec![]).unwrap();
        assert_eq!(result.len(), 1);
        let ref dao = result[0];
        let create_sql: String = dao.get("sql");
        match Sqlite::extract_comments(&create_sql) {
            Ok((_table_comment, column_comments)) => {
                debug!("column_comments: {:?}", column_comments);
                column_comments
            }
            Err(_) => {
                BTreeMap::new()
            }
        }
    }

    fn get_column_comment(&self,
                          column_comments: &BTreeMap<String, Option<String>>,
                          column: &str)
                          -> Option<String> {
        match column_comments.get(column) {
            Some(comment) => comment.clone(),
            None => None,
        }

    }
    fn get_column_foreign(&self, all_foreign: &[Foreign], column: &str) -> Option<Foreign> {
        debug!("foreign: {:#?} ", all_foreign);
        for foreign in all_foreign {
            if foreign.column == column {
                return Some(foreign.clone());
            }
        }
        None
    }
}

impl Database for Sqlite {
    fn version(&self) -> Result<String, DbError> {
        let sql = "SELECT sqlite_version() AS version";
        let dao = try!(self.execute_sql_with_one_return(sql, &vec![]));
        match dao {
            Some(dao) => Ok(dao.get("version")),
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
        unimplemented!()
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
            SqlOption::UsesNumberedParam,  // uses numbered parameters
            SqlOption::SupportsCTE,
        ]
    }

    fn insert(&self, query: &Query) -> Result<Dao, DbError> {
        let sql_frag = self.build_insert(query, BuildMode::Standard);
        match self.execute_sql_with_one_return(&sql_frag.sql, &sql_frag.params) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(DbError::new("No result from insert")),
            Err(e) => Err(e),
        }
    }
    fn update(&self, _query: &Query) -> Dao {
        unimplemented!()
    }
    fn delete(&self, _query: &Query) -> Result<usize, String> {
        unimplemented!()
    }

    /// sqlite does not return the columns mentioned in the query,
    /// you have to specify it yourself
    /// TODO: found this
    /// http://jgallagher.github.io/rusqlite/rusqlite/struct.SqliteStatement.html#method.column_names
    fn execute_sql_with_return(&self, sql: &str, params: &[Value]) -> Result<Vec<Dao>, DbError> {
        debug!("SQL: \n{}", sql);
        debug!("param: {:?}", params);
        let conn = self.get_connection();
        let mut stmt = conn.prepare(sql).unwrap();
        let mut daos = vec![];
        let param = self.from_rust_type_tosql(params);
        let mut columns = vec![];
        for c in stmt.column_names() {
            columns.push(c.to_owned());
        }
        debug!("columns : {:?}", columns);
        let rows = try!(stmt.query(&param));
        for row in rows {
            let row = try!(row);
            let mut index = 0;
            let mut dao = Dao::new();
            for col in &columns {
                let rtype = self.from_sql_to_rust_type(&row, index);
                debug!("{:?}", rtype);
                dao.set_value(col, rtype);
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
            return Ok(Some(dao[0].clone()));
        } else {
            return Ok(None);
        }
    }

    /// generic execute sql which returns not much information,
    /// returns only the number of affected records or errors
    /// can be used with DDL operations (CREATE, DELETE, ALTER, DROP)
    fn execute_sql(&self, sql: &str, params: &[Value]) -> Result<usize, DbError> {
        debug!("SQL: \n{}", sql);
        debug!("param: {:?}", params);
        let to_sql_types = self.from_rust_type_tosql(params);
        let conn = self.get_connection();
        let result = conn.execute(sql, &to_sql_types);
        match result {
            Ok(result) => {
                Ok(result as usize)
            }
            Err(e) => {
                Err(DbError::new(&format!("Something is wrong, {}", e)))
            }
        }
    }

}

impl DatabaseDDL for Sqlite {
    fn create_schema(&self, _schema: &str) {
        panic!("sqlite does not support schema")
    }

    fn drop_schema(&self, _schema: &str) {
        panic!("sqlite does not support schema")
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
                w.ln_tab();
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
		let mut do_comma = true;//there has been colcommentsumns mentioned
		for c in &table.columns {
			if let Some(ref foreign) = c.foreign {
				if do_comma {
					w.commasp();
				} else {
					do_comma = true;
				}
				w.ln_tab();
				w.append("FOREIGN KEY");
				w.append(&format!("({})", c.name));
				w.append(" REFERENCES ");
				w.append(&format!("{}", foreign.table));
				w.append(&format!("({})", foreign.column));
			}
		}
        w.ln();
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

impl DatabaseDev for Sqlite {
    fn get_table_sub_class(&self, _schema: &str, _table: &str) -> Vec<String> {
        unimplemented!()
    }

    fn get_parent_table(&self, _schema: &str, _table: &str) -> Option<String> {
        unimplemented!()
    }

    fn get_table_metadata(&self, schema: &str, table: &str, _is_view: bool) -> Table {
        debug!("extracting table meta data in sqlite");
        let sql = format!("PRAGMA table_info({});", table);
        let result = self.execute_sql_with_return(&sql, &vec![]);
        debug!("result: {:#?}", result);
        match result {
            Ok(result) => {
                let foreign = self.get_foreign_keys(schema, table);
                let table_comment = self.get_table_comment(schema, table);
                let column_comments = self.get_column_comments(schema, table);

                let mut columns = vec![];
                for r in result {
                    let column: String = r.get("name");
                    let db_data_type: String = r.get("type");
                    let default_value: String = r.get("dflt_value");
                    let not_null: String = r.get("notnull");
                    let pk: String = r.get("pk");
                    debug!("column: {}", column);
                    debug!("data_type: {}", db_data_type);
                    debug!("not null: {}", not_null);
                    debug!("pk: {}", pk);
                    debug!("default_value: {}", default_value);

                    let column_comment = self.get_column_comment(&column_comments, &column);
                    let column_foreign = self.get_column_foreign(&foreign, &column);
					let (_, data_type) = self.dbtype_to_rust_type(&db_data_type);
                    let column = Column {
                        table: Some(table.to_owned()),
                        name: column,
                        data_type: data_type,
                        db_data_type: db_data_type,
                        is_primary: pk != "0",
                        is_unique: false,
                        default: Some(Operand::Value(Value::String(default_value))),
                        comment: column_comment,
                        not_null: not_null != "0",
                        is_inherited: false,
                        foreign: column_foreign,
                    };
                    columns.push(column);
                }
                Table {
                    schema: None,
                    name: table.to_owned(),
                    parent_table: None,
                    sub_table: vec![],
                    comment: table_comment,
                    columns: columns,
                    is_view: false,
                }
            }
            Err(e) => {
                panic!("No matching table found {}", e);
            }
        }
    }

    fn get_all_tables(&self) -> Vec<(String, String, bool)> {
        let sql = "SELECT type, name, tbl_name, sql FROM sqlite_master WHERE type = 'table'";
        let result = self.execute_sql_with_return(&sql, &vec![]);
        match result {
            Ok(result) => {
                let mut tables: Vec<(String, String, bool)> = Vec::new();
                for r in result {
                    let schema = "".to_owned();
                    let table: String = r.get("tbl_name");
                    let is_view = false;
                    tables.push((schema, table, is_view))
                }
                tables
            }
            Err(e) => {
                panic!("Unable to get tables due to {}", e)
            }
        }
    }

    fn get_inherited_columns(&self, _schema: &str, _table: &str) -> Vec<String> {
        vec![]
    }

    fn dbtype_to_rust_type(&self, _db_type: &str) -> (Vec<String>, Type) {
        unimplemented!()
    }

    fn rust_type_to_dbtype(&self, _rust_type: &Type) -> String {
        unimplemented!()
    }
}


#[test]
fn test_comment_extract() {
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
    let _ = Sqlite::extract_comments(create_sql);
}
