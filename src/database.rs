use query::Query;
use table::Table;
use dao::{Dao, DaoResult, Value};
use writer::SqlFrag;
use query::{Connector, Equality, Operand, Field};
use query::{Direction, Modifier, NullsWhere, JoinType};
use query::{Filter, Condition};
use query::Range;
use std::error::Error;
use std::fmt;
use r2d2;
use postgres::error::Error as PgError;
use postgres::error::ConnectError as PgConnectError;
#[cfg(feature = "mysql")]
use mysql::error::MyError;
use regex::Error as RegexError;
#[cfg(feature = "sqlite")]
use rusqlite::Error as SqliteError;
use platform::PlatformError;
use dao::Type;
use query::source::{SourceField, QuerySource};
use query::{Select,Insert,Update,Delete};
use query::query::Data;
use query::ColumnName;


/// SqlOption, contains the info about the features and quirks of underlying database
#[derive(PartialEq)]
pub enum SqlOption {
    /// use the numbered parameters, as the case with rust-postgres
    UsesNumberedParam,
    /// sqlite, jdbc
    UsesQuestionMark,
    /// postgresql supports returning clause on insert and update
    SupportsReturningClause,
    /// support CTE (common table expression ie. WITH) (postgresql, sqlite)
    SupportsCTE,
    /// supports inheritance (postgresql)
    SupportsInheritance,
    /// whether the database uses schema (postgresl, oracle)
    UsesSchema,
    /// wheter the returned rows in a query included Meta columns for easy extraction of records
    /// (postgres returns this), sqlite does not return meta columns, so you have to extract it by index yourself.
    ReturnMetaColumns,
}

/// specifies if the sql will be build in debug mode for debugging purposed
#[derive(PartialEq)]
#[derive(Clone)]
pub enum BuildMode {
    /// build in debug mode
    Debug,
    /// build in standard mode
    Standard,
}

#[derive(Debug)]
pub enum DbError {
    Error(String),
    PoolError(r2d2::InitializationError),
    PlatformError(PlatformError),
}

impl DbError {
    pub fn new(description: &str) -> Self {
        DbError::Error(description.to_owned())
    }
}

impl Error for DbError {
    fn description(&self) -> &str {
        match *self {
            DbError::Error(ref description) => description,
            DbError::PoolError(ref err) => err.description(),
            DbError::PlatformError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            DbError::Error(_) => None,
            DbError::PoolError(ref err) => Some(err),
            DbError::PlatformError(ref err) => Some(err),
        }
    }
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DbError::PoolError(ref err) => write!(f, "Pool error: {}", err),
            DbError::PlatformError(ref err) => write!(f, "PostgreSQL error: {}", err),
            DbError::Error(_) => write!(f, "{}", self.description()),
        }
    }
}

impl From<r2d2::InitializationError> for DbError {
    fn from(err: r2d2::InitializationError) -> Self {
        DbError::PoolError(err)
    }
}

impl From<PlatformError> for DbError {
    fn from(err: PlatformError) -> Self {
        DbError::PlatformError(err)
    }
}

impl From<RegexError> for DbError {
    fn from(err: RegexError) -> Self {
        DbError::new(err.description())
    }
}


impl From<PgError> for DbError {
    fn from(err: PgError) -> Self {
        DbError::PlatformError(From::from(err))
    }
}


impl From<PgConnectError> for DbError {
    fn from(err: PgConnectError) -> Self {
        DbError::PlatformError(From::from(err))
    }
}
#[cfg(feature = "mysql")]
impl From<MyError> for DbError {
    fn from(err: MyError) -> Self {
        DbError::PlatformError(From::from(err))
    }
}

#[cfg(feature = "sqlite")]
impl From<SqliteError> for DbError {
    fn from(err: SqliteError) -> Self {
        DbError::PlatformError(From::from(err))
    }
}

/// Generic Database interface
/// This is the database interface which will should be implemented to you the specifics of each database platform
/// At least all methods on this trait should be implemented for target deployment database
/// A lower level API for manipulating objects in the database
///
/// TODO: acquire only a connection until a query is about to be executed.
/// generating query don't really need database connection just yet.

pub trait Database {
    /// return the version of the database
    /// lower version of database has fewer supported features
    fn version(&self) -> Result<String, DbError>;

    /// begin database transaction
    fn begin(&self);

    /// commit database transaction
    fn commit(&self);

    /// rollback data changes executed prior to calling the begin method
    fn rollback(&self);


    /// select
    /// returns an array to the qualified records
    fn select(&self, query: &Select) -> Result<DaoResult, DbError> {
        self.execute_with_return(query)
    }

    /// insert
    /// insert an object, returns the inserted Dao value
    /// including the value generated via the defaults
    fn insert(&self, query: &Insert) -> Result<Dao, DbError> {
        let sql_frag = self.build_insert(query, &BuildMode::Standard);
        match self.execute_sql_with_one_return(&sql_frag.sql, &sql_frag.params) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(DbError::new("No result from insert")),
            Err(e) => Err(e),
        }
    }

    /// update
    /// returns the updated Dao
    fn update(&self, query: &Update) -> Result<Dao,DbError>{
        let sql_frag = self.build_update(query, &BuildMode::Standard);
        match self.execute_sql_with_one_return(&sql_frag.sql, &sql_frag.params) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(DbError::new("No result from insert")),
            Err(e) => Err(e),
        }
    }


    /// execute query with return dao,
    /// use the enumerated column for data extraction when db doesn't support returning the records column names
    fn execute_with_return(&self, query: &Select) -> Result<DaoResult, DbError> {
        let sql_frag = &self.build_select(query, &BuildMode::Standard);
        let result = try!(self.execute_sql_with_return(&sql_frag.sql, &sql_frag.params));
        if query.enable_query_stat{
            let (page, page_size, total) = try!(self.get_query_stats(query));
            let dao_result = DaoResult {
                dao: result,
                renamed_columns: query.get_renamed_columns(),
                total: total,
                page: page,
                page_size: page_size,
            };
            Ok(dao_result)
        }else{
            let dao_result = DaoResult {
                dao: result,
                renamed_columns: query.get_renamed_columns(),
                total: None,
                page: None,
                page_size: None,
            };
            Ok(dao_result)
        }
    }

    /// get the query stats page, page_size and the total records
    fn get_query_stats(&self,
                       query: &Select)
                       -> Result<(Option<usize>, Option<usize>, Option<usize>), DbError> {
        let page = if let Some(limit) = query.range.limit {
            if let Some(offset) = query.range.offset {
                Some(offset / limit)
            } else {
                None
            }
        } else {
            None
        };

        let mut count_query = query.to_owned();
        count_query.enumerated_fields = vec![];//remove the enumerated fields
        count_query.column("COUNT(*) AS COUNT");
        count_query.order_by = vec![];
        count_query.range = Range::new();//remove the range
        let debug_sql = &self.build_select(&count_query, &BuildMode::Debug);
        println!("STAT QUERY: {}", debug_sql);
        let count_result = try!(self.execute_with_one_return(&count_query));
        println!("range: {:#?}", query.range);
        println!("count result {:#?}", count_result);
        let total = if let Some(count_result) = count_result {
            let value = count_result.get("count");
            match value {
                Some(&Value::U64(v)) => Some(v as usize),
                Some(&Value::I64(v)) => Some(v as usize),
                Some(&Value::U32(v)) => Some(v as usize),
                Some(&Value::I32(v)) => Some(v as usize),
                _ => None,
            }
        } else {
            None
        };
        Ok((page, query.range.limit, total))
    }

    /// execute query with 1 return dao
    fn execute_with_one_return(&self, query: &Select) -> Result<Option<Dao>, DbError> {
        let sql_frag = &self.build_select(query, &BuildMode::Standard);
        self.execute_sql_with_one_return(&sql_frag.sql, &sql_frag.params)
    }

    /// delete records
    /// returns the number of deleted records
    fn delete(&self, query: &Delete) -> Result<usize, DbError> {
        let sql_frag = &self.build_delete(query, &BuildMode::Standard);
        self.execute_sql(&sql_frag.sql, &sql_frag.params)
    }

    /// execute insert with returning clause, update with returning clause
    fn execute_sql_with_return(&self, sql: &str, params: &[Value]) -> Result<Vec<Dao>, DbError>;

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

    /// everything else, no required return other than error or affected number of records
    fn execute_sql(&self, sql: &str, param: &[Value]) -> Result<usize, DbError>;

    /// build a query, return the sql string and the parameters.
    /// use by select to build the select query
    /// build all types of query
    /// TODO: need to supply the number of parameters where to start the numbering of the number parameters
    fn build_query(&self, query: &Query, build_mode: &BuildMode) -> SqlFrag {
        match *query {
            Query::Select(ref select) => self.build_select(select, build_mode),
            Query::Insert(ref insert) => self.build_insert(insert, build_mode),
            Query::Update(ref update) => self.build_update(update, build_mode),
            Query::Delete(ref delete) => self.build_delete(delete, build_mode),
        }
    }

    /// build operand, i.e: columns, query, function, values
    fn build_operand(&self, w: &mut SqlFrag, use_complete_name: bool, operand: &Operand) {
        match *operand {
            Operand::ColumnName(ref column_name) => {
                if use_complete_name {
                    w.append(&column_name.column);
                } else {
                    w.append(&column_name.complete_name());
                }
            }
            Operand::QuerySource(ref query_source) => {
                self.build_query_source(w, query_source);
            }
            Operand::Value(ref value) => {
                w.parameter(value.clone());
            }
            Operand::Vec(ref operands) => {
                let mut do_comma = false;
                if !operands.is_empty() {
                    w.append("(");
                    for op in operands {
                        if do_comma {
                            w.commasp();
                        } else {
                            do_comma = true;
                        }
                        self.build_operand(w, use_complete_name, op);
                    }
                    w.append(")");
                }
            }
            Operand::None => (), //dont do anything
        }
    }

    fn build_condition(&self, w: &mut SqlFrag, use_complete_name: bool, cond: &Condition) {
        self.build_operand(w, use_complete_name, &cond.left);
        w.append(" ");
        match cond.equality {
            Equality::EQ => {
                w.append("= ");
                self.build_operand(w, use_complete_name, &cond.right);
            }
            Equality::NEQ => {
                w.append("!= ");
                self.build_operand(w, use_complete_name, &cond.right);
            }
            Equality::LT => {
                w.append("< ");
                self.build_operand(w, use_complete_name, &cond.right);
            }
            Equality::LTE => {
                w.append("<= ");
                self.build_operand(w, use_complete_name, &cond.right);
            }
            Equality::GT => {
                w.append("> ");
                self.build_operand(w, use_complete_name, &cond.right);
            }
            Equality::GTE => {
                w.append(">= ");
                self.build_operand(w, use_complete_name, &cond.right);
            }
            Equality::IN => {
                w.append("IN ");
                self.build_operand(w, use_complete_name, &cond.right);
            }
            Equality::NOT_IN => {
                w.append("NOT IN ");
                self.build_operand(w, use_complete_name, &cond.right);
            }
            Equality::LIKE => {
                w.append("LIKE ");
                self.build_operand(w, use_complete_name, &cond.right);
            }
            Equality::ILIKE => {
                w.append("ILIKE ");
                self.build_operand(w, use_complete_name, &cond.right);
            }
            Equality::IS_NOT_NULL => {
                w.append("IS NOT NULL");
            }

            Equality::IS_NULL => {
                w.append("IS NULL");
            }
        }
    }

    fn build_field(&self, w: &mut SqlFrag, use_complete_name: bool, field: &Field) {
        self.build_operand(w, use_complete_name, &field.operand);
        match field.name {
            Some(ref name) => {
                w.append(" AS ");
                w.append(name);
            }
            None => (),
        }
    }

    fn build_query_source(&self,
                          w: &mut SqlFrag,
                          query_source: &QuerySource) {
        match *query_source {
            QuerySource::TableName(ref table_name) => {
                if self.sql_options().contains(&SqlOption::UsesSchema) {
                    w.append(&table_name.complete_name());
                } else {
                    w.append(&table_name.name);
                }
            }
            QuerySource::Function(ref function) => {
                w.sp();
                w.append(&function.function);
                w.append("(");
                let mut do_comma = false;
                for param in &function.params {
                    if do_comma {
                        w.commasp();
                    } else {
                        do_comma = true;
                    }
                    self.build_operand(w, false, param);
                }
                w.append(")");
            }
            QuerySource::Query(ref _q) => {
                let sql_frag = &self.build_select(&_q, &w.build_mode);
                w.append(&sql_frag.sql);
            }
        }
    }

    fn build_source_field(&self,
                          w: &mut SqlFrag,
                          source_field: &SourceField) {
        self.build_query_source(w, &source_field.source);
        match source_field.rename {
            Some(ref rename) => {
                w.append(" AS ");
                w.append(rename);
            }
            None => (),
        }
    }

    fn build_filter(&self, w: &mut SqlFrag, use_complete_name: bool, filter: &Filter) {
        if !filter.sub_filters.is_empty() {
            w.append("( ");
        }
        self.build_condition(w, use_complete_name, &filter.condition);
        w.sp();
        for filt in &filter.sub_filters {
            match filt.connector {
                Connector::And => {
                    w.append("AND ");
                }
                Connector::Or => {
                    w.append("OR ");
                }
            }
            self.build_filter(w, use_complete_name, filt);// build sub filters as well
        }
        if !filter.sub_filters.is_empty() {
            w.append(" )");
        }
    }

    /// build the filter clause or the where clause of the query
    /// TODO: add the sub filters
    fn build_filters(&self, w: &mut SqlFrag, use_complete_name: bool, filters: &[Filter]) {
        let mut do_and = false;
        for filter in filters {
            if do_and {
                w.left_river(" AND ");
            } else {
                do_and = true;
            }
            self.build_filter(w, use_complete_name, filter);
        }
    }

    /// build the enumerated, distinct, *, columns
    fn build_enumerated_fields(&self,
                               w: &mut SqlFrag,
                               use_complete_name: bool, 
                               enumerated_fields: &[Field]) {
        let mut do_comma = false;
        let mut cnt = 0;
        for field in enumerated_fields {
            if do_comma {
                w.commasp();
            } else {
                do_comma = true;
            }
            cnt += 1;
            if cnt % 4 == 0 {
                // break at every 4 columns to encourage sql tuning/revising
                w.left_river("");
            }
            self.build_field(w, use_complete_name, field);
        }
    }

    /// build the select statment from the query object
    fn build_select(&self, query: &Select, build_mode: &BuildMode) -> SqlFrag {
        let use_complete_name = query.joins.is_empty();
        let mut w = SqlFrag::new(self.sql_options(), build_mode);
        w.left_river("SELECT");
        self.build_enumerated_fields(&mut w, use_complete_name, &query.enumerated_fields); //TODO: add support for column_sql, fields, functions
        w.left_river("FROM");

        assert!(!query.from.is_empty(),
                "There should be table, query, function to select from");
        let mut do_comma = false;
        for field in &query.from {
            if do_comma {
                w.commasp();
            } else {
                do_comma = true;
            }
            self.build_source_field(&mut w, field);
        }
        if !query.joins.is_empty() {
            for join in &query.joins {
                match join.modifier {
                    Some(ref modifier) => {
                        match *modifier {
                            Modifier::LEFT => w.right_river("LEFT "),
                            Modifier::RIGHT => w.right_river("RIGHT "),
                            Modifier::FULL => w.right_river("FULL "),
                        };
                    }
                    None => (),
                }
                match join.join_type {
                    Some(ref join_type) => {
                        match *join_type {
                            JoinType::CROSS => w.right_river("CROSS "),
                            JoinType::INNER => w.right_river("INNER "),
                            JoinType::OUTER => w.right_river("OUTER "),
                            JoinType::NATURAL => w.right_river("NATURAL "),
                        };
                    }
                    None => (),
                }
                w.append("JOIN ");
                w.append(&join.table_name.complete_name());
                w.right_river("ON ");
                self.build_filter(&mut w, use_complete_name, &join.on);
            }
        }

        if !query.filters.is_empty() {
            w.left_river("WHERE ");
            self.build_filters(&mut w, use_complete_name, &query.filters);
        }

        if !query.group_by.is_empty() {
            w.left_river("GROUP BY ");
            let mut do_comma = false;
            for operand in &query.group_by {
                if do_comma {
                    w.comma();
                } else {
                    do_comma = true;
                }
                self.build_operand(&mut w, use_complete_name, operand);
                w.append(" ");
            }
        }

        if !query.having.is_empty() {
            w.left_river("HAVING ");
            let mut do_comma = false;
            for hav in &query.having {
                if do_comma {
                    w.commasp();
                } else {
                    do_comma = true;
                }
                self.build_filter(&mut w, use_complete_name, hav);
            }
        }

        if !query.order_by.is_empty() {
            w.left_river("ORDER BY ");
            let mut do_comma = false;
            for order in &query.order_by {
                if do_comma {
                    w.commasp();
                } else {
                    do_comma = true;
                }
                self.build_operand(&mut w, use_complete_name, &order.operand);
                match &order.direction {
                    &Some(ref direction) => {
                        match direction {
                            &Direction::ASC => w.append(" ASC"),
                            &Direction::DESC => w.append(" DESC"),
                        }
                    }
                    &None => w.append(""),
                };
                match &order.nulls_where {
                    &Some(ref nulls_where) => {
                        match nulls_where {
                            &NullsWhere::FIRST => w.append(" NULLS FIRST"),
                            &NullsWhere::LAST => w.append(" NULLS LAST"),
                        }
                    }
                    &None => w.append(""),
                };
            }
        }
        match query.range.limit {
            Some(limit) => {
                w.left_river("LIMIT ");
                w.append(&format!("{}", limit));
            }
            None => (),
        }
        match query.range.offset {
            Some(offset) => {
                w.left_river("OFFSET ");
                w.append(&format!("{}", offset));
            }
            None => (),
        }
        w
    }

    /// TODO: when the number of values is greater than the number of columns
    /// wrap it into another set and make sure the values are in multiples of the the n columns
    /// http://www.postgresql.org/docs/9.0/static/dml-insert.html
    fn build_insert(&self, query: &Insert, build_mode: &BuildMode) -> SqlFrag {
        let use_complete_name = false;
        let mut w = SqlFrag::new(self.sql_options(), build_mode);
        w.left_river("INSERT");
        w.append("INTO ");
        if self.sql_options().contains(&SqlOption::UsesSchema) {
            w.append(&query.into.complete_name());
        } else {
            w.append(&query.into.name);
        }
        w.append("( ");
        self.build_column_names(&mut w, use_complete_name, &query.columns);
        w.append(" ) ");
        match query.data{
            Data::Values(ref values) => {
                w.left_river("VALUES");
                w.append("(");
                let mut do_comma = false;
                for vo in values {
                    if do_comma {
                        w.commasp();
                    } else {
                        do_comma = true;
                    }
                    self.build_operand(&mut w, use_complete_name, vo);
                }
                w.append(") ");
            },
            Data::Query(ref data_query) => {
               let sql_frag = self.build_select(data_query, build_mode);
               w.append(&sql_frag.sql);
            }
        }
        if !query.return_columns.is_empty() {
            if self.sql_options().contains(&SqlOption::SupportsReturningClause) {
                w.left_river("RETURNING");
                self.build_column_names(&mut w, use_complete_name, &query.return_columns);
            }
        }
        w.ln();
        w
    }

    fn build_column_names(&self, w: &mut SqlFrag, use_complete_name: bool, column_names: &Vec<ColumnName>){
        let mut do_comma = false;
        for c in column_names {
            if do_comma{w.commasp();}else{do_comma = true;}
            if use_complete_name{
                w.append(&c.complete_name());
            }else{
                w.append(&c.column);
            }
        }
    }


    fn build_update(&self, query: &Update,  build_mode: &BuildMode) -> SqlFrag {
        let use_complete_name = false;
        let mut w = SqlFrag::new(self.sql_options(), build_mode);
        w.left_river("UPDATE ");
        w.append(&query.table.complete_name());
        let mut do_comma = false;
        if !query.columns.is_empty() {
            w.left_river("SET ");
        }
        let mut column_index = 0;
        assert_eq!(query.columns.len(), query.values.len());
        for ec in &query.columns {
            if do_comma {
                w.commasp();
            } else {
                do_comma = true;
            }
            w.append(&ec.column);
            w.append(" = ");
            let value = &query.values[column_index];
            self.build_operand(&mut w, use_complete_name, value);
            column_index += 1;
        }

        if !query.filters.is_empty() {
            w.left_river("WHERE ");
            self.build_filters(&mut w, use_complete_name, &query.filters);
        }
        if !query.return_columns.is_empty() {
            if self.sql_options().contains(&SqlOption::SupportsReturningClause) {
                w.left_river("RETURNING ");
                self.build_column_names(&mut w, use_complete_name, &query.return_columns) 
            }
        }
        w
    }

    fn build_delete(&self, query: &Delete, build_mode: &BuildMode) -> SqlFrag {
        let use_complete_name = false;
        let mut w = SqlFrag::new(self.sql_options(), build_mode);
        w.left_river("DELETE FROM ");
        w.append(&query.from_table.complete_name());
        if !query.filters.is_empty() {
            w.left_river("WHERE ");
            self.build_filters(&mut w, use_complete_name, &query.filters);
        }
        w
    }

    fn sql_options(&self) -> Vec<SqlOption>;
}


/// Deployment Database should implement this trait,
/// to enable automated installation of the app, regardless what database platform
/// the app is developed from.
pub trait DatabaseDDL {
    /// ///////////////////////////////////////
    /// The following methods involves DDL(Data definition language) operation
    // //////////////////////////////////////
    /// create a database schema
    fn create_schema(&self, schema: &str);

    /// drop the database schema
    fn drop_schema(&self, schema: &str);

    /// create a database table based on the Model Definition
    fn create_table(&self, model: &Table);

    /// build sql for create table
    fn build_create_table(&self, table: &Table) -> SqlFrag;

    /// rename table, in the same schema
    fn rename_table(&self, table: &Table, new_tablename: String);

    /// drop table
    fn drop_table(&self, table: &Table);

    /// set the foreign key constraint of a table
    fn set_foreign_constraint(&self, model: &Table);

    /// set the primary key constraint of a table
    fn set_primary_constraint(&self, model: &Table);
}


/// implement this for database that you use as your development platform, to extract meta data information
/// about the tables and their relationship to each other
pub trait DatabaseDev {
    /// /////////////////////////////////////
    /// Database interface use for the development process
    // //////////////////////////////////////////
    /// applicable to later version of postgresql where there is inheritance
    fn get_table_sub_class(&self, schema: &str, table: &str) -> Vec<String>;

    fn get_parent_table(&self, schema: &str, table: &str) -> Option<String>;


    fn get_row_count_estimate(&self, schema: &str, table: &str) -> Option<usize>;

    /// /
    /// Build the Table object based on the extracted meta data info from database
    /// This is queries directly from the database, so this will be costly. Only used this on initialization processes
    ///
    fn get_table_metadata(&self, schema: &str, table: &str, is_view: bool) -> Table;

    /// get all the tables in this database (schema, table, is_view)
    fn get_all_tables(&self) -> Vec<(String, String, bool)>;

    /// get the inherited columns of this table
    fn get_inherited_columns(&self, schema: &str, table: &str) -> Vec<String>;

    /// get the equivalent postgresql database data type to rust data type
    /// returns (module, type)
    fn dbtype_to_rust_type(&self, db_type: &str) -> (Vec<String>, Type);

    fn rust_type_to_dbtype(&self, rust_type: &Type) -> String;
}
