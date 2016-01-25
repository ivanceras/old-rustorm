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

pub use self::column_name::{ColumnName,ToColumnName};
pub use self::table_name::{TableName,ToTableName};
pub use self::filter::{Filter,Condition,Equality,Connector,HasEquality};
pub use self::builder::QueryBuilder;
pub use self::function::COUNT;
pub use self::function::Function;
pub use self::join::{Join,JoinType,Modifier};
pub use self::operand::Operand;
pub use self::order::{Order,ToOrder,HasDirection,NullsWhere,Direction};






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


#[derive(Debug)]
#[derive(Clone)]
pub struct Field {
    /// the field
    pub operand: Operand,
    /// when renamed as field
    pub name: Option<String>,
}

impl Field {

    fn rename(&self) -> Field {
        match self.operand {
            Operand::ColumnName(ref column_name) => {
                let rename = column_name.default_rename();
                Field {
                    operand: Operand::ColumnName(column_name.clone()),
                    name: Some(rename),
                }
            }
            _ => unimplemented!(),
        }
    }
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
pub struct Page{
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}

impl Page{
    fn to_limit(&self)->Limit{
        let offset = if self.page.is_some() && self.page_size.is_some(){
            Some(self.page.unwrap() *  self.page_size.unwrap())
        }else{
            None
        };

        Limit{
            limit: self.page_size,
            offset: offset,
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Default)]
#[derive(Clone)]
pub struct Limit{
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Range{
    Page(Page),
    Limit(Limit),
}

impl Range{
    
    fn get_limit(&self)->Limit{
        match self{
            &Range::Page(ref page) => {
                page.to_limit()
            },
            &Range::Limit(ref limit) => {
                limit.clone()
            }
        }
    }
    
}


#[derive(Debug)]
#[derive(Clone)]
pub struct Query {

    ///sql type determine which type of query to form, some fields are not applicable to other types of query
    pub sql_type:SqlType,

    /// whether to select the records distinct
    pub distinct: bool,

    /// whether to enumate all columns in involved models
    pub enumerate_all: bool,

    pub declared_query: BTreeMap<String, Query>,

    ///fields can be functions, column sql query, and even columns
    /// TODO; merge enumerated column to this, add a builder for fields
    pub enumerated_fields:Vec<Field>,

    /// specify to use distinct ON set of columns
    pub distinct_on_columns:Vec<String>,
    
    /// where the focus of values of column selection
    /// this is the table to insert to, update to delete, create, drop
    /// whe used in select, this is the
    /// pub from_table:Option<TableName>,

    /// from field, where field can be a query, table, column, or function
	/// TODO: Use Vec<Field> here since from can have multiple tables
    //pub from:Option<Box<Field>>,
    pub from:Vec<Field>,

    /// joining multiple tables
    pub joins:Vec<Join>,

    /// filter records, ~ where statement of the query
    pub filters:Vec<Filter>,


    /// ordering of the records via the columns specified
    /// TODO: needs to support expressions/functions too
    pub order_by:Vec<Order>,

    /// grouping columns to create an aggregate
    pub group_by: Vec<Operand>,

    /// having field, 
    pub having: Vec<Filter>,

    /// exclude the mention of the columns in the SQL query, useful when ignoring changes in update/insert records
    pub excluded_columns:Vec<ColumnName>,

    /// caters both limit->offset and page->page_size
    /// setting page and page_size can not interchange the order
    pub range: Option<Range>,
    /// The data values, used in bulk inserting, updating,
    pub values:Vec<Operand>,

    /// the returning clause of the query when supported,
    pub enumerated_returns: Vec<Field>,
}

impl Query {

    /// the default query is select
    pub fn new() -> Self {
        Query {
            sql_type: SqlType::SELECT,
            distinct: false,
            enumerate_all: false,
            declared_query: BTreeMap::new(),
            enumerated_fields: vec![],
            distinct_on_columns: vec![],
            filters: vec![],
            joins: vec![],
            order_by: vec![],
            group_by: vec![],
            having: vec![],
            excluded_columns: vec![],
            range: None,
            from: vec![],
            values: vec![],
            enumerated_returns: vec![],
        }
    }


    pub fn enumerate_all(&mut self) -> &mut Self {
        self.enumerate_all = true;
       	self
    }

    pub fn all(&mut self) -> &mut Self {
        self.column("*")
    }

    fn enumerate(&mut self, column_name: ColumnName) -> &mut Self {
        let operand = Operand::ColumnName(column_name);
        let field = Field {
            operand: operand,
            name: None,
        };
        self.enumerated_fields.push(field);
        self
    }

    /// all enumerated columns shall be called from this
    /// any conflict of columns from some other table will be automatically renamed
    /// columns that are not conflicts from some other table,
    /// but is the other conflicting column is not explicityly enumerated will not be renamed
    ///
    pub fn column(&mut self, column: &str) -> &mut Self {
        let column_name = ColumnName::from_str(column);
        self.enumerate(column_name);
        self
    }


    pub fn columns(&mut self, columns: Vec<&str>) -> &mut Self {
        for c in columns {
            self.column(c);
        }
        self
    }


    /// exclude columns when inserting/updating data
    /// also ignores the column when selecting records
    /// useful for manipulating thin records by excluding huge binary blobs such as images
    pub fn exclude_column(&mut self, column: &str) -> &mut Self {
        let c = ColumnName::from_str(column);
        self.excluded_columns.push(c);
        self
    }
    pub fn exclude_columns(&mut self, columns: Vec<&str>) -> &mut Self {
        for c in columns {
            self.exclude_column(c);
        }
        self
    }

    pub fn distinct_on_columns(&mut self, columns: &Vec<String>) -> &mut Self {
        let columns = columns.clone();
        for c in columns {
            self.distinct_on_columns.push(c);
        }
        self
    }

    /// when paging multiple records
    pub fn set_page(&mut self, page: usize) -> &mut Self {
        let new_range = match self.range{
            Some(ref range) => {
                match range{
                    &Range::Page(ref p) => {
                        //replacing the page
                        Some(Range::Page(Page{page:Some(page), page_size: p.page_size}))
                    },
                    &Range::Limit(_) => {
                        panic!("Do not mix page->page_size with limit and offset");
                    }
                }
            },
            None => {
                // a new range with 0 page_size
                Some(Range::Page(Page{page:Some(page), page_size:None}))
            }
        };
        self.range = new_range;
        self
    }
    
    pub fn set_page_size(&mut self, page_size: usize) -> &mut Self {
        let new_range = match self.range{
            Some(ref range) => {
                match range{
                    &Range::Page(ref p) => {
                        //replacing the page_size
                        Some(Range::Page(Page{page:p.page, page_size: Some(page_size)}))
                    },
                    &Range::Limit(_) => {
                        panic!("Do not mix page->page_size with limit and offset");
                    }
                    
                }
            },
            None => {
                // a new range with page 1
                Some(Range::Page(Page{page:None, page_size:Some(page_size)}))
            }
        };
        self.range = new_range;
        self
    }
    
    pub fn get_limit(&self)->Option<Limit>{
        match self.range{
            Some(ref range) => Some(range.get_limit()),
            None => None
        }
    }

    /// enumerate only the columns that is coming from this table
    /// this will invalidate enumerate_all
    pub fn only_from(&mut self, table: &ToTableName) -> &mut Self {
        self.enumerate_all = false;
        self.enumerate_from_table(&table.to_table_name());
		self
    }
    


    /// a query to query from
    /// use WITH (query) t1 SELECT from t1 declaration in postgresql, sqlite
    /// use SELECT FROM (query) in oracle, mysql, others
    /// alias of the table
    pub fn from_query(&mut self, query: Query, alias: &str) -> &mut Self {
        let operand = Operand::Query(query);
        let field = Field {
            operand: operand,
            name: Some(alias.to_owned()),
        };
        self.from_field(field)
    }

    pub fn from_field(&mut self, field: Field) -> &mut Self {
        self.from.push(field);
        self
    }


    pub fn get_from_table(&self) -> Option<&TableName> {
        if !self.from.is_empty() {
			match self.from[0].operand{
				Operand::TableName(ref table_name) => return Some(table_name),
				_ => return None
			} 
        }
        None
    }


	fn add_order(&mut self, operand: Operand, direction:Option<Direction>, nulls_where:Option<NullsWhere>)->&mut Self{
        self.order_by.push(Order{
			operand: operand,
			direction: direction,
			nulls_where: nulls_where,
		});
        self
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
    fn rename_fields(&mut self, column: &str) -> &mut Self {
        let matched_indexes = self.match_fields_indexes(column);
        for index in matched_indexes {
            let field = self.enumerated_fields.remove(index);//remove it
            let field = field.rename(); //rename it
            self.enumerated_fields.insert(index, field); //insert it back to the same location
        }
        self
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
    /// TODO: renamed conflicting enumerated columns
    /// if no enumerated fields and no excluded columns
    /// do a select all
    pub fn finalize(&mut self) -> &Self {

        let involved_models = self.get_involved_tables();
        if involved_models.len() > 1 {
            //enumerate all columns when there is a join
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

    fn enumerate_from_table(&mut self, table: &TableName) {
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
    fn rename_conflicting_columns(&mut self) -> &mut Self {
        let conflicts = self.get_conflicting_columns();
        for c in conflicts {
            self.rename_fields(&c);
        }
        self
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

    fn remove_from_enumerated(&mut self, column_name: &ColumnName) -> &mut Self {
        let index = self.index_of_field(column_name);
        if let Some(idx) = index {
            self.enumerated_fields.remove(idx);
        }
        self
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


    pub fn add_filter(&mut self, filter: Filter) -> &mut Self {
        self.filters.push(filter);
        self
    }

    pub fn WHERE(&mut self, filter: Filter) -> &mut Self {
        self.filters.push(filter);
        self
    }
    pub fn add_filters(&mut self, filters: Vec<Filter>) -> &mut Self {
        for f in filters {
            self.add_filter(f);
        }
        self
    }

    pub fn filter(&mut self, column: &str, equality: Equality, value: &ToValue) -> &mut Self {
        self.add_filter(Filter::new(column, equality, value))
    }

    // attach and clause
    pub fn AND(&mut self, filter: Filter) -> &mut Self {
        // TODO: if last function call is filter, then append to filter
        // if last function call is join, then append to join
        self.filters.push(filter);
        self
    }

    // attach or clause
    pub fn OR(&mut self, column: &str, equality: Equality, value: &ToValue) -> &mut Self {
        self.filters.last_mut().unwrap().or(column, equality, value);
        self
    }

    /// column = value
    pub fn filter_eq(&mut self, column: &str, value: &ToValue) -> &mut Self {
        self.add_filter(Filter::new(column, Equality::EQ, value))
    }
    /// column < value
    pub fn filter_lt(&mut self, column: &str, value: &ToValue) -> &mut Self {
        self.add_filter(Filter::new(column, Equality::LT, value))
    }
    /// column <= value
    pub fn filter_lte(&mut self, column: &str, value: &ToValue) -> &mut Self {
        self.add_filter(Filter::new(column, Equality::LTE, value))
    }

    /// column > value
    pub fn filter_gt(&mut self, column: &str, value: &ToValue) -> &mut Self {
        self.add_filter(Filter::new(column, Equality::GT, value))
    }
    /// column <= value
    pub fn filter_gte(&mut self, column: &str, value: &ToValue) -> &mut Self {
        self.add_filter(Filter::new(column, Equality::GTE, value))
    }

    pub fn add_value_operand(&mut self, value: Operand) -> &mut Self {
        self.values.push(value);
        self
    }

    pub fn add_value(&mut self, value: &Value) -> &mut Self {
        let operand = Operand::Value(value.clone());
        self.values.push(operand);
        self
    }
    pub fn value(&mut self, value: &ToValue) -> &mut Self {
        let value = value.to_db_type();
        self.add_value(&value)
    }

    /// set a value of a column when inserting/updating records
    pub fn SET(&mut self, column: &str, value: &ToValue) -> &mut Self {
        self.column(column);
        self.value(value)
    }
	pub fn set_value(&mut self, column: &str, value: &Value) -> &mut Self{
		self.column(column);
		self.add_value(value)
	}

    pub fn return_all(&mut self) -> &mut Self {
        self.enumerate_column_as_return("*")
    }

    pub fn returns(&mut self, columns: Vec<&str>) -> &mut Self {
        for c in columns {
            self.enumerate_column_as_return(c);
        }
        self
    }

    pub fn enumerate_column_as_return(&mut self, column: &str) -> &mut Self {
        let column_name = ColumnName::from_str(column);
        let operand = Operand::ColumnName(column_name);
        let field = Field {
            operand: operand,
            name: None,
        };
        self.enumerated_returns.push(field);
        self
    }

    /// build the query only, not executed, useful when debugging
    pub fn build(&mut self, db: &Database) -> SqlFrag {
        self.finalize();
        db.build_query(self, BuildMode::Standard)
    }

	/// Warning: don't use this in production
	pub fn debug_build(&mut self, db: &Database)-> SqlFrag{
		self.finalize();
		db.build_query(self, BuildMode::Debug)
	}

	///retrieve a generic types, type is unknown at runtime
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
