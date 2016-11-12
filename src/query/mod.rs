use dao::{Value, ToValue};
use table::Table;
use std::collections::BTreeMap;
use database::Database;
use dao::DaoResult;
use dao::IsDao;
use dao::Dao;
use table::IsTable;
use writer::SqlFrag;
use std::fmt;
use database::DbError;
use database::BuildMode;

pub mod column_name;
pub mod table_name;
pub mod filter;
pub mod builder;
pub mod function;
pub mod join;
pub mod operand;
pub mod order;
pub mod field;
pub mod source;

pub use self::column_name::{ColumnName, ToColumnName};
pub use self::table_name::{TableName, ToTableName};
pub use self::filter::{Filter, Condition, Equality, Connector, HasEquality};
pub use self::builder::QueryBuilder;
pub use self::function::COUNT;
pub use self::function::Function;
pub use self::join::{Join, JoinType, Modifier};
pub use self::operand::Operand;
pub use self::order::{Order, ToOrder, HasDirection, NullsWhere, Direction};
pub use self::field::{Field, ToField};
pub use self::source::SourceField;
pub use self::source::{QuerySource, ToSourceField};




/// Could have been SqlAction
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum SqlType {
    // DML
    SELECT,
    INSERT,
    UPDATE,
    DELETE,
}



/// Query Error
pub enum Error {
    NoTableSpecified(String),
    NoColumnSpecified(String),
    SqlError(String),
}



#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Default)]
#[derive(Clone)]
pub struct Range {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl Range {
    pub fn new() -> Self {
        Range {
            limit: None,
            offset: None,
        }
    }

    pub fn set_limit(&mut self, limit: usize) {
        self.limit = Some(limit);
    }
    pub fn set_offset(&mut self, offset: usize) {
        self.offset = Some(offset);
    }
}


#[derive(Debug)]
#[derive(Clone)]
pub struct DeclaredQuery{
    name: String,
    fields: Vec<String>,
    query: Query,
    is_recursive: bool
}


#[derive(Debug)]
#[derive(Clone)]
pub struct Query {
    /// sql type determine which type of query to form, some fields are not applicable to other types of query
    pub sql_type: SqlType,

    /// whether to select the records distinct
    pub distinct: bool,

    /// whether to enumate all columns in involved models
    pub enumerate_all: bool,

    pub declared_query: Vec<DeclaredQuery>,

    /// fields can be functions, column sql query, and even columns
    pub enumerated_fields: Vec<Field>,

    /// specify to use distinct ON set of columns
    pub distinct_on_columns: Vec<String>,

    /// where the focus of values of column selection
    /// this is the table to insert to, update to delete, create, drop
    /// whe used in select, this is the
    /// pub from_table:Option<TableName>,

    /// from field, where field can be a query, table, or function
    pub from: Vec<SourceField>,

    /// joining multiple tables
    pub joins: Vec<Join>,

    /// filter records, ~ where statement of the query
    pub filters: Vec<Filter>,

    /// ordering of the records via the columns specified
    pub order_by: Vec<Order>,

    /// grouping columns to create an aggregate
    pub group_by: Vec<Operand>,

    /// having field,
    pub having: Vec<Filter>,

    /// exclude the mention of the columns in the SQL query, useful when ignoring changes in update/insert records
    pub excluded_columns: Vec<ColumnName>,

    /// caters both limit->offset and page->page_size
    /// setting page and page_size can not interchange the order
    pub range: Range,
    /// The data values, used in bulk inserting, updating,
    pub values: Vec<Operand>,

    /// the returning clause of the query when supported,
    pub enumerated_returns: Vec<Field>,

    /// enable query stats
    pub enable_query_stat: bool,
}

impl Query {
    /// the default query is select
    pub fn new() -> Self {
        Query {
            sql_type: SqlType::SELECT,
            distinct: false,
            enumerate_all: false,
            declared_query: vec![],
            enumerated_fields: vec![],
            distinct_on_columns: vec![],
            filters: vec![],
            joins: vec![],
            order_by: vec![],
            group_by: vec![],
            having: vec![],
            excluded_columns: vec![],
            range: Range::new(),
            from: vec![],
            values: vec![],
            enumerated_returns: vec![],
            enable_query_stat: true,
        }
    }


    pub fn select() -> Self {
        let mut q = Query::new();
        q.sql_type = SqlType::SELECT;
        q
    }

    pub fn insert() -> Self {
        let mut q = Query::new();
        q.sql_type = SqlType::INSERT;
        q
    }
    pub fn update() -> Self {
        let mut q = Query::new();
        q.sql_type = SqlType::UPDATE;
        q
    }
    pub fn delete() -> Self {
        let mut q = Query::new();
        q.sql_type = SqlType::DELETE;
        q
    }

    pub fn enumerate_all(&mut self) {
        self.enumerate_all = true;
    }

    pub fn all(&mut self) {
        self.column("*");
    }
    pub fn select_all() -> Self {
        let mut q = Self::select();
        q.all();
        q
    }

    fn enumerate(&mut self, column_name: ColumnName) {
        let operand = Operand::ColumnName(column_name);
        let field = Field {
            operand: operand,
            name: None,
        };
        self.enumerated_fields.push(field);
    }

    /// all enumerated columns shall be called from this
    /// any conflict of columns from some other table will be automatically renamed
    /// columns that are not conflicts from some other table,
    /// but is the other conflicting column is not explicityly enumerated will not be renamed
    ///
    pub fn column(&mut self, column: &str) {
        let column_name = ColumnName::from_str(column);
        self.enumerate(column_name);
    }


    pub fn columns(&mut self, columns: Vec<&str>) {
        for c in columns {
            self.column(c);
        }
    }


    /// exclude columns when inserting/updating data
    /// also ignores the column when selecting records
    /// useful for manipulating thin records by excluding huge binary blobs such as images
    pub fn exclude_column(&mut self, column: &str) {
        let c = ColumnName::from_str(column);
        self.excluded_columns.push(c);
    }
    pub fn exclude_columns(&mut self, columns: Vec<&str>) {
        for c in columns {
            self.exclude_column(c);
        }
    }

    pub fn distinct_on_columns(&mut self, columns: &Vec<String>) {
        let columns = columns.clone();
        for c in columns {
            self.distinct_on_columns.push(c);
        }
    }

    pub fn value(&mut self, value: &ToValue) {
        let value = value.to_db_type();
        self.values.push(Operand::Value(value));
    }

    /// set a value of a column when inserting/updating records
    pub fn set(&mut self, column: &str, value: &ToValue) {
        self.column(column);
        self.value(value);
    }
    pub fn set_limit(&mut self, limit: usize) {
        self.range.set_limit(limit);
    }
    pub fn set_offset(&mut self, offset: usize) {
        self.range.set_offset(offset);
    }

    pub fn get_range(&self) -> Range {
        self.range.to_owned()
    }

    /// enumerate only the columns that is coming from this table
    /// this will invalidate enumerate_all
    pub fn only_from(&mut self, table: &ToTableName) {
        self.enumerate_all = false;
        self.enumerate_from_table(&table.to_table_name());
    }



    /// a query to query from
    /// use WITH (query) t1 SELECT from t1 declaration in postgresql, sqlite
    /// use SELECT FROM (query) in oracle, mysql, others
    /// alias of the table
    pub fn from_query(&mut self, query: Query, alias: &str) {
        let sf = SourceField {
            source: QuerySource::Query(query),
            rename: Some(alias.to_owned()),
        };
        self.from.push(sf);
    }

    pub fn from(&mut self, to_source_field: &ToSourceField) {
        self.from.append(&mut to_source_field.to_source_field());
    }

    pub fn table(&mut self, to_source_field: &ToSourceField) {
        self.from(to_source_field);
    }
    /// returns the first table in the from clause
    pub fn get_from_table(&self) -> Option<TableName> {
        for fr in &self.from {
            match &fr.source {
                &QuerySource::TableName(ref table_name) => {
                    return Some(table_name.to_owned());
                }
                _ => {} 
            }
        }
        None
    }


    /// get the indexes of the fields that matches the the column name
    fn match_fields_indexes(&self, column: &str) -> Vec<usize> {
        let mut indexes = vec![];
        let mut cnt = 0;
        for field in &self.enumerated_fields {
            if let Operand::ColumnName(ref column_name) = field.operand {
                if column_name.column == column {
                    indexes.push(cnt);
                }
            }
            cnt += 1;
        }
        indexes
    }

    /// take the enumerated field that is a column that matched the name
    fn rename_fields(&mut self, column: &str) {
        let matched_indexes = self.match_fields_indexes(column);
        for index in matched_indexes {
            let field = self.enumerated_fields.remove(index);//remove it
            let field = field.rename(); //rename it
            self.enumerated_fields.insert(index, field); //insert it back to the same location
        }
    }

    pub fn get_involved_tables(&self) -> Vec<TableName> {
        let mut tables = vec![];
        let from_table = self.get_from_table();
        if let Some(from) = from_table {
            tables.push(from.clone());
        }
        for j in &self.joins {
            if !tables.contains(&&j.table_name) {
                tables.push(j.table_name.clone());
            }
        }
        tables
    }

    /// preprocess the missing fields of the query,
    /// such as mentioning the columns of the from_table
    /// enumerate the columns of the involved tables
    /// skipping those which are explicitly ignored
    /// the query will then be built and ready to be executed
    /// if no enumerated fields and no excluded columns
    /// do a select all
    pub fn finalize(&mut self) -> &Self {

        let involved_models = self.get_involved_tables();
        if involved_models.len() > 1 {
            // enumerate all columns when there is a join
            if self.enumerate_all {
                self.enumerate_involved_tables_columns(&involved_models);
            }
            self.rename_conflicting_columns(); // rename an enumerated columns that conflicts
        }
        let excluded_columns = &self.excluded_columns.clone();
        for i in excluded_columns {
            self.remove_from_enumerated(&i);
        }
        if self.excluded_columns.is_empty() && self.enumerated_fields.is_empty() {
            self.all();
        }
        self
    }

    fn enumerate_involved_tables_columns(&mut self, involved_models: &Vec<TableName>) {
        for m in involved_models {
            for c in &m.columns {
                self.enumerate(c.clone());
            }
        }
    }

    pub fn enumerate_from_table(&mut self, table: &TableName) {
        for c in &table.columns {
            self.enumerate(c.clone());
        }
    }

    fn get_renamed_fields(&self) -> Vec<&Field> {
        let mut renamed = vec![];
        for field in &self.enumerated_fields {
            if field.name.is_some() {
                renamed.push(field);
            }
        }
        renamed
    }

    /// return the list of renamed columns, used in dao conversion to struc types
    pub fn get_renamed_columns(&self) -> Vec<(ColumnName, String)> {
        let mut renamed_columns = vec![];
        let renamed_fields = self.get_renamed_fields();
        for field in &renamed_fields {
            if let Operand::ColumnName(ref column_name) = field.operand {
                if let Some(ref rename) = field.name {
                    renamed_columns.push((column_name.clone(), rename.to_owned()));
                }
            }
        }
        renamed_columns
    }

    /// determine which columns are conflicting and rename it accordingly
    /// rename only the columns that are in the enumerated list
    fn get_conflicting_columns(&self) -> Vec<String> {
        let mut conflicts = vec![];
        let enumerated_columns = self.get_enumerated_columns();
        for c in &enumerated_columns {
            for d in &enumerated_columns {
                if c != d && c.is_conflicted(d) {
                    conflicts.push(c.column.to_owned());
                }
            }
        }
        conflicts
    }
    /// rename the fields that has a conflicting column
    fn rename_conflicting_columns(&mut self) {
        let conflicts = self.get_conflicting_columns();
        for c in conflicts {
            self.rename_fields(&c);
        }
    }

    /// used by removed_from_enumerated
    fn index_of_field(&self, column: &ColumnName) -> Option<usize> {
        let mut cnt = 0;
        for field in &self.enumerated_fields {
            if let Operand::ColumnName(ref column_name) = field.operand {
                if column_name == column {
                    return Some(cnt);
                }
            }
            cnt += 1;
        }
        None
    }

    fn remove_from_enumerated(&mut self, column_name: &ColumnName) {
        let index = self.index_of_field(column_name);
        if let Some(idx) = index {
            self.enumerated_fields.remove(idx);
        }
    }

    /// return the list of enumerated columns
    /// will be used for updating records
    pub fn get_enumerated_columns(&self) -> Vec<&ColumnName> {
        let mut columns = vec![];
        for field in &self.enumerated_fields {
            if let Operand::ColumnName(ref column_name) = field.operand {
                columns.push(column_name);
            }
        }
        columns
    }


    pub fn add_filter(&mut self, filter: Filter) {
        self.filters.push(filter);
    }
    pub fn add_filters(&mut self, filters: &Vec<Filter>) {
        self.filters.extend_from_slice(filters)
    }

    /// column = value
    pub fn filter_eq(&mut self, column: &str, value: &ToValue) {
        self.add_filter(Filter::new(column, Equality::EQ, value));
    }
    /// column < value
    pub fn filter_lt(&mut self, column: &str, value: &ToValue) {
        self.add_filter(Filter::new(column, Equality::LT, value));
    }
    /// column <= value
    pub fn filter_lte(&mut self, column: &str, value: &ToValue) {
        self.add_filter(Filter::new(column, Equality::LTE, value));
    }

    /// column > value
    pub fn filter_gt(&mut self, column: &str, value: &ToValue) {
        self.add_filter(Filter::new(column, Equality::GT, value));
    }
    /// column <= value
    pub fn filter_gte(&mut self, column: &str, value: &ToValue) {
        self.add_filter(Filter::new(column, Equality::GTE, value));
    }


    pub fn return_all(&mut self) {
        self.enumerate_column_as_return("*");
    }

    pub fn returns(&mut self, columns: Vec<&str>) {
        for c in columns {
            self.enumerate_column_as_return(c);
        }
    }

    pub fn enumerate_column_as_return(&mut self, column: &str) {
        let column_name = ColumnName::from_str(column);
        let operand = Operand::ColumnName(column_name);
        let field = Field {
            operand: operand,
            name: None,
        };
        self.enumerated_returns.push(field);
    }

    /// build the query only, not executed, useful when debugging
    pub fn build(&mut self, db: &Database) -> SqlFrag {
        self.finalize();
        db.build_query(self, BuildMode::Standard)
    }

    /// Warning: don't use this in production
    pub fn debug_build(&mut self, db: &Database) -> SqlFrag {
        self.finalize();
        db.build_query(self, BuildMode::Debug)
    }

    /// retrieve a generic types, type is unknown at runtime
    /// expects a return, such as select, insert/update with returning clause
    pub fn retrieve(&mut self, db: &Database) -> Result<DaoResult, DbError> {
        self.finalize();
        db.execute_with_return(self)
    }

    /// expects a return, such as select, insert/update with returning clause
    /// no casting of data to structs is done
    /// This is used when retrieving multiple models in 1 query, then casting the records to its equivalent structs
    pub fn retrieve_one(&mut self, db: &Database) -> Result<Option<Dao>, DbError> {
        self.finalize();
        db.execute_with_one_return(self)
    }

    /// delete, update without caring for the return
    pub fn execute(&mut self, db: &Database) -> Result<usize, DbError> {
        self.finalize();
        db.execute(self)
    }

    /// execute the query, then convert the result
    pub fn collect<T: IsDao + IsTable>(&mut self, db: &Database) -> Result<Vec<T>, DbError> {
        let result = try!(self.retrieve(db));
        Ok(result.cast())
    }

    /// execute the query then collect only 1 record
    pub fn collect_one<T: IsDao + IsTable>(&mut self, db: &Database) -> Result<T, DbError> {
        let result = try!(self.retrieve(db));
        match result.cast_one() {
            Some(res) => Ok(res),
            None => Err(DbError::new("No entry to collect found.")),
        }
    }
}
