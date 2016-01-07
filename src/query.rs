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

#[derive(Debug)]
#[derive(Clone)]
pub enum JoinType {
    CROSS,
    INNER,
    OUTER,
    NATURAL,
}
#[derive(Debug)]
#[derive(Clone)]
pub enum Modifier {
    LEFT,
    RIGHT,
    FULL,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Join {
    pub modifier: Option<Modifier>,
    pub join_type: Option<JoinType>,
    pub table_name: TableName,
    pub column1: Vec<String>,
    pub column2: Vec<String>,
}
#[derive(Debug)]
#[derive(Clone)]
pub enum Direction {
    ASC,
    DESC,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum NullsWhere {
    FIRST,
    LAST,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Order{
	pub operand: Operand,
	pub direction: Option<Direction>,
	pub nulls_where: Option<NullsWhere>,
}

///
/// Filter struct merged to query
///
#[derive(Debug)]
#[derive(Clone)]
pub enum Connector {
    And,
    Or,
}

#[derive(Debug)]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub enum Equality {
    EQ, // EQUAL,
    NEQ, // NOT_EQUAL,
    LT, // LESS_THAN,
    LTE, // LESS_THAN_OR_EQUAL,
    GT, // GREATER_THAN,
    GTE, // GREATER_THAN_OR_EQUAL,
    IN,
    NOT_IN, // NOT_IN,
    LIKE,
    ILIKE, //add ILIKE
    IS_NOT_NULL, // NOT_NULL,
    IS_NULL, // IS_NULL,
}

/// function in a sql statement
#[derive(Debug)]
#[derive(Clone)]
pub struct Function {
    pub function: String,
    pub params: Vec<Operand>,
}

/// Operands can be columns, functions, query or value types
#[derive(Debug)]
#[derive(Clone)]
pub enum Operand {
    ColumnName(ColumnName),
    TableName(TableName),
    Function(Function),
    Query(Query),
    Value(Value),
    Vec(Vec<Operand>),
}

/// expression has left operand,
/// equality and right operand
#[derive(Debug)]
#[derive(Clone)]
pub struct Condition {
    pub left: Operand,
    pub equality: Equality,
    pub right: Operand,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Filter {
    pub connector: Connector,
    /// TODO: maybe renamed to LHS, supports functions and SQL
    pub condition: Condition,
    pub sub_filters: Vec<Filter>, 
}

impl Filter {

    /// user friendly, commonly use API
    pub fn new(column: &str, equality: Equality, value: &ToValue) -> Self {
        let right = Operand::Value(value.to_db_type());
        Filter {
            connector: Connector::And,
            condition: Condition {
                left: Operand::ColumnName(ColumnName::from_str(column)),
                equality: equality,
                right: right,
            },
            sub_filters: vec![],
        }
    }

    /// user friendly, commonly use API
    pub fn with_value(column: &str, equality: Equality, value: Value) -> Self {
        let right = Operand::Value(value);
        Filter {
            connector: Connector::And,
            condition: Condition {
                left: Operand::ColumnName(ColumnName::from_str(column)),
                equality: equality,
                right: right,
            },
            sub_filters: vec![],
        }
    }


    /// not very commonly used, offers enough flexibility
    pub fn bare_new(left: Operand, equality: Equality, right: Operand) -> Self {
        Filter {
            connector: Connector::And,
            condition: Condition {
                left: left,
                equality: equality,
                right: right,
            },
            sub_filters: vec![],
        }
    }


    pub fn is_null(column: &str) -> Self {
        Filter::new(column, Equality::IS_NULL, &())
    }
    pub fn is_not_null(column: &str) -> Self {
        Filter::new(column, Equality::IS_NOT_NULL, &())
    }

    pub fn and(&mut self, column: &str, equality: Equality, value: &ToValue) -> &mut Self {
        let mut filter = Filter::new(column, equality, value);
        filter.connector = Connector::And;
        self.sub_filters.push(filter);
        self
    }

    pub fn or(&mut self, column: &str, equality: Equality, value: &ToValue) -> &mut Self {
        let mut filter = Filter::new(column, equality, value);
        filter.connector = Connector::Or;
        self.sub_filters.push(filter);
        self
    }

    pub fn or_filter(&mut self, filter: Filter) -> &mut Self {
        let mut filter = filter.clone();
        filter.connector = Connector::Or;
        self.sub_filters.push(filter);
        self
    }
    pub fn and_filter(&mut self, filter: Filter) -> &mut Self {
        let mut filter = filter.clone();
        filter.connector = Connector::And;
        self.sub_filters.push(filter);
        self
    }
}

/// Could have been SqlAction
#[derive(Debug)]
#[derive(Clone)]
pub enum SqlType {
    // DML
    SELECT,
    INSERT,
    UPDATE,
    DELETE,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(RustcEncodable, RustcDecodable)]
pub struct ColumnName {
    pub column: String,
    pub table: Option<String>,
    // //optional schema, if ever there are same tables resideing in  different schema/namespace
    pub schema: Option<String>,
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

impl ColumnName {

    pub fn from_str(column: &str) -> Self {
        if column.contains(".") {
            let splinters = column.split(".").collect::<Vec<&str>>();
            assert!(splinters.len() == 2, "There should only be 2 splinters");
            let table_split = splinters[0].to_owned();
            let column_split = splinters[1].to_owned();
            ColumnName {
                column: column_split.to_owned(),
                table: Some(table_split.to_owned()),
                schema: None,
            }
        } else {
            ColumnName {
                column: column.to_owned(),
                table: None,
                schema: None,
            }
        }
    }

    fn default_rename(&self) -> String {
        match self.table {
            Some(ref s) => format!("{}.{}", s, self.column),
            None => panic!("Unable to rename {} since table is not specified",
                           self.column),
        }
    }

    /// table name and column name
    pub fn complete_name(&self) -> String {
        match self.table {
            Some(ref s) => format!("{}.{}", s, self.column),
            None => self.column.to_owned(),
        }
    }
    /// includes the schema, table name and column name
    pub fn super_complete_name(&self) -> String {
        match self.schema {
            Some(ref s) => format!("{}.{}", s, self.complete_name()),
            None => self.complete_name(),
        }
    }

    /// is this column conflicts the other column
    /// conflicts means, when used both in a SQL query, it will result to ambiguous columns
    fn is_conflicted(&self, other: &ColumnName) -> bool {
        self.column == other.column
    }
}

impl fmt::Display for ColumnName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.complete_name())
    }
}

impl PartialEq for ColumnName {
    fn eq(&self, other: &Self) -> bool {
        self.column == other.column && self.table == other.table
    }

    fn ne(&self, other: &Self) -> bool {
        self.column != other.column || self.table != other.table || self.schema != other.schema
    }
}


#[derive(Clone)]
#[derive(Debug)]
pub struct TableName {
    pub schema: Option<String>,
    pub name: String,
    /// optional columns needed when rename for conflicting columns are needed
    pub columns: Vec<ColumnName>,
}

impl TableName {

    pub fn from_str(str: &str) -> Self {
        if str.contains(".") {
            let splinters = str.split(".").collect::<Vec<&str>>();
            assert!(splinters.len() == 2, "There should only be 2 splinters");
            let schema_split = splinters[0].to_owned();
            let table_split = splinters[1].to_owned();

            TableName {
                schema: Some(schema_split),
                name: table_split,
                columns: vec![],
            }

        } else {
            TableName {
                schema: None,
                name: str.to_owned(),
                columns: vec![],
            }
        }
    }

    pub fn complete_name(&self) -> String {
        match self.schema {
            Some (ref schema) => format!("{}.{}", schema, self.name),
            None => self.name.to_owned(),
        }
    }
}

impl PartialEq for TableName {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.schema == other.schema
    }

    fn ne(&self, other: &Self) -> bool {
        self.name != other.name || self.schema != other.schema
    }
}

impl fmt::Display for TableName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.complete_name())
    }
}

/// convert str, IsTable to TableName
pub trait ToTableName {

    fn to_table_name(&self) -> TableName;

}

impl <'a>ToTableName for &'a str {

    fn to_table_name(&self) -> TableName {
        TableName::from_str(self)
    }
}

impl ToTableName for str {

    fn to_table_name(&self) -> TableName {
        TableName::from_str(self)
    }
}

impl ToTableName for String {

    fn to_table_name(&self) -> TableName {
        TableName::from_str(self)
    }
}

impl ToTableName for Table {

    /// contain the columns for later use when renaming is necessary
    fn to_table_name(&self) -> TableName {
        let mut columns = vec![];
        for c in &self.columns {
            let column_name = ColumnName {
                schema: self.schema.clone(),
                table: Some(self.name.to_owned()),
                column: c.name.to_owned(),
            };
            columns.push(column_name);
        }
        TableName {
            schema: self.schema.clone(),
            name: self.name.to_owned(),
            columns: columns,
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
    pub from:Option<Box<Field>>,

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
            from: None,
            values: vec![],
            enumerated_returns: vec![],
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

    /// add DISTINCT ie: SELECT DISTINCT
    pub fn distinct(&mut self) -> &mut Self {
        self.distinct = true;
        self
    }

    pub fn select_all() -> Self {
        let mut q = Self::select();
        q.all();
        q
    }
    pub fn enumerate_all() -> Self {
        let mut q = Self::select();
        q.enumerate_all = true;
        q
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

    pub fn group_by(&mut self, columns: Vec<&str>) -> &mut Self {
        for c in columns {
            let column_name = ColumnName::from_str(c);
            let operand = Operand::ColumnName(column_name);
            self.group_by.push(operand);
        }
        self
    }

    pub fn having(&mut self, column: &str, equality: Equality, value: &ToValue) -> &mut Self {
        let filter = Filter::new(column, equality, value);
        self.having.push(filter);
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
    
    pub fn limit(&mut self, n: usize)->&mut Self{
        self.set_page_size(n)
    }

    pub fn get_limit(&self)->Option<Limit>{
        match self.range{
            Some(ref range) => Some(range.get_limit()),
            None => None
        }
    }

    /// A more terse way to write the query
    /// only 1 table is supported yet
    pub fn from(&mut self, table: &ToTableName) -> &mut Self {
        let table_name = table.to_table_name();
        let operand = Operand::TableName(table_name);
        let field = Field {
            operand: operand,
            name: None,
        };
        self.from_field(field)
    }
    /// enumerate only the columns that is coming from this table
    /// this will invalidate enumerate_all
    pub fn only_from(&mut self, table: &ToTableName) -> &mut Self {
        self.enumerate_all = false;
        self.enumerate_from_table(&table.to_table_name());
        self.from(table)
    }
    pub fn from_table(&mut self, table: &str) -> &mut Self {
        self.from(&table)
    }
    /// `into` is used in rust, os settled with `into_`
    pub fn into_(&mut self, table: &ToTableName) -> &mut Self {
        self.sql_type = SqlType::INSERT;
        self.from(table)
    }
    /// can not use into since it's rust .into built-in (owned)
    pub fn into_table(&mut self, table: &str) -> &mut Self {
        self.into_(&table)
    }
    /// can be used in behalf of into_, from,
    pub fn table(&mut self, table: &ToTableName) -> &mut Self {
        self.from(table)
    }

    /// if the database support CTE declareted query i.e WITH,
    /// then this query will be declared
    /// if database doesn't support WITH queries, then this query will be
    /// wrapped in the from_query
    /// build a builder for this
    pub fn declare_query(&mut self, query: Query, alias: &str) -> &mut Self {
        self.declared_query.insert(alias.to_owned(), query);
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
        self.from = Some(Box::new(field));
        self
    }

    pub fn get_from_table(&self) -> Option<&TableName> {
        if let Some(ref field) = self.from {
            if let Operand::TableName(ref table_name) = field.operand {
                return Some(table_name);
            }
        }
        None
    }

    /// join a table on this query
    ///
    pub fn join(&mut self, join: Join) -> &mut Self {
        self.joins.push(join);
        self
    }

    /// join a table on this query
    ///
    pub fn left_join_table(&mut self, table: &str, column1: &str, column2: &str) -> &mut Self {
        self.left_join(&table, column1, column2)
    }
    pub fn left_join(&mut self, table: &ToTableName, column1: &str, column2: &str) -> &mut Self {
        let join = Join {
            modifier: Some(Modifier::LEFT),
            join_type: None,
            table_name: table.to_table_name(),
            column1: vec![column1.to_owned()],
            column2: vec![column2.to_owned()],
        };
        self.join(join)
    }
    pub fn right_join_table(&mut self, table: &str, column1: &str, column2: &str) -> &mut Self {
        self.right_join(&table, column1, column2)
    }
    pub fn right_join(&mut self, table: &ToTableName, column1: &str, column2: &str) -> &mut Self {
        let join = Join {
            modifier: Some(Modifier::RIGHT),
            join_type: None,
            table_name: table.to_table_name(),
            column1: vec![column1.to_owned()],
            column2: vec![column2.to_owned()],
        };
        self.join(join)
    }
    pub fn full_join_table(&mut self, table: &str, column1: &str, column2: &str) -> &mut Self {
        self.full_join(&table, column1, column2)
    }
    pub fn full_join(&mut self, table: &ToTableName, column1: &str, column2: &str) -> &mut Self {
        let join = Join {
            modifier: Some(Modifier::FULL),
            join_type: None,
            table_name: table.to_table_name(),
            column1: vec![column1.to_owned()],
            column2: vec![column2.to_owned()],
        };
        self.join(join)
    }

    pub fn inner_join_table(&mut self, table: &str, column1: &str, column2: &str) -> &mut Self {
        self.inner_join(&table, column1, column2)
    }
    pub fn inner_join(&mut self, table: &ToTableName, column1: &str, column2: &str) -> &mut Self {
        let join = Join {
            modifier: None,
            join_type: Some(JoinType::INNER),
            table_name: table.to_table_name(),
            column1: vec![column1.to_owned()],
            column2: vec![column2.to_owned()],
        };
        self.join(join)
    }

	fn add_order(&mut self, operand: Operand, direction:Option<Direction>, nulls_where:Option<NullsWhere>)->&mut Self{
        self.order_by.push(Order{
			operand: operand,
			direction: direction,
			nulls_where: nulls_where,
		});
        self
	}
	pub fn order_by(&mut self, column: &str, direction: Option<Direction>, nulls_where: Option<NullsWhere>)->&mut Self{
		let operand = Operand::ColumnName(ColumnName::from_str(column));
		self.add_order(operand, direction, nulls_where)
	}
    ///ascending orderby of this column
    pub fn asc(&mut self, column: &str) -> &mut Self {
        self.order_by(column,  Some(Direction::ASC), None);
        self
    }
    ///ascending orderby of this column
    pub fn asc_nulls_first(&mut self, column: &str) -> &mut Self {
        self.order_by(column, Some(Direction::ASC), Some(NullsWhere::FIRST));
        self
    }
    ///ascending orderby of this column
    pub fn asc_nulls_last(&mut self, column: &str) -> &mut Self {
        self.order_by(column, Some(Direction::ASC), Some(NullsWhere::LAST));
        self
    }
    ///descending orderby of this column
    pub fn desc(&mut self, column: &str) -> &mut Self {
        self.order_by(column, Some(Direction::DESC), None);
        self
    }
    ///descending orderby of this column
    pub fn desc_nulls_first(&mut self, column: &str) -> &mut Self {
        self.order_by(column, Some(Direction::DESC), Some(NullsWhere::FIRST));
        self
    }
    ///descending orderby of this column
    pub fn desc_nulls_last(&mut self, column: &str) -> &mut Self {
        self.order_by(column, Some(Direction::DESC), Some(NullsWhere::LAST));
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
    pub fn and(&mut self, column: &str, equality: Equality, value: &ToValue) -> &mut Self {
        self.filters.last_mut().unwrap().and(column, equality, value);
        self
    }

    // attach or clause
    pub fn or(&mut self, column: &str, equality: Equality, value: &ToValue) -> &mut Self {
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
    pub fn set(&mut self, column: &str, value: &ToValue) -> &mut Self {
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
