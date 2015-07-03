use dao::{Type, ToType};
use table::{Table, Column};
use std::collections::BTreeMap;
use database::Database;
use dao::DaoResult;
use dao::IsDao;
use dao::Dao;
use table::IsTable;
use writer::SqlFrag;

#[derive(Debug)]
#[derive(Clone)]
pub enum JoinType{
    CROSS,
    INNER,
    OUTER,
}
#[derive(Debug)]
#[derive(Clone)]
pub enum Modifier{
    LEFT,
    RIGHT,
    FULL,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Join{
    pub modifier:Option<Modifier>,
    pub join_type:JoinType,
    pub table_name:TableName,
    pub column1:Vec<String>,
    pub column2:Vec<String>
}
#[derive(Debug)]
#[derive(Clone)]
pub enum Direction{
    ASC,
    DESC,
}


////
/// Filter struct merged to query
/// 
#[derive(Debug)]
#[derive(Clone)]
pub enum Connector{
    And,
    Or
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Equality{
    EQ, //EQUAL,
    NE, //NOT_EQUAL,
    LT, //LESS_THAN,
    LTE, //LESS_THAN_OR_EQUAL,
    GT, //GREATER_THAN,
    GTE, //GREATER_THAN_OR_EQUAL,
    IN,
    NOTIN,//NOT_IN,
    LIKE,
    NULL,
    NOTNULL,//NOT_NULL,
    ISNULL,//IS_NULL,
}

/// function in a sql statement
#[derive(Debug)]
#[derive(Clone)]
pub struct Function{
    pub function:String,
    pub params:Vec<Operand>,
}

/// Operands can be columns, functions, query or value types
#[derive(Debug)]
#[derive(Clone)]
pub enum Operand{
    ColumnName(ColumnName),
    TableName(TableName),
    Function(Function),
    Query(Query),
    Value(Type),
    Vec(Vec<Operand>),
}

/// expression has left operand,
/// equality and right operand
#[derive(Debug)]
#[derive(Clone)]
pub struct Condition{
    pub left_operand:Operand,
    pub equality:Equality,
    pub right_operand:Operand,
}

/// TODO: support for functions on columns
/// TODO: need to merge with Expr
#[derive(Debug)]
#[derive(Clone)]
pub struct Filter{
    pub connector:Connector,
    /// TODO: maybe renamed to LHS, supports functions and SQL
    pub condition: Condition,
    pub subfilters:Vec<Filter>
}

impl Filter{

    pub fn new(column:&str, equality:Equality, value:&ToType)->Self{
        let right_operand = Operand::Value(value.to_db_type());
        Filter{
            connector:Connector::And,
            condition: Condition{left_operand:
                        Operand::ColumnName(ColumnName::from_str(column)),
                        equality:equality,
                        right_operand:right_operand},
            subfilters:vec![],
        }
    }
    
    pub fn and(&mut self, column:&str, equality:Equality, value:&ToType)->&mut Self{
        let mut filter = Filter::new(column, equality, value);
        filter.connector = Connector::And;
        self.subfilters.push(filter);
        self
    }
    
    pub fn or(&mut self, column:&str, equality:Equality, value:&ToType)->&mut Self{
        let mut filter = Filter::new(column, equality, value);
        filter.connector = Connector::Or;
        self.subfilters.push(filter);
        self
    }
    
}

/// Could have been SqlAction
#[derive(Debug)]
#[derive(Clone)]
pub enum SqlType{
    //DML
    SELECT,
    INSERT,
    UPDATE,
    DELETE,
}

#[derive(Clone)]
#[derive(Debug)]
pub struct ColumnName{
    pub column:String,
    pub table:Option<String>,
    ////optional schema, if ever there are same tables resideing in  different schema/namespace
    pub schema:Option<String>,
    /// as rename
    pub rename:Option<String>
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Field{
    /// the field
    pub operand:Operand,
    /// when renamed as field
    pub name:Option<String>,
}


impl ColumnName{

    fn from_column(column:&Column, table:&Table)->Self{
        ColumnName{
            column: column.name.to_string(),
            table: Some(table.name.to_string()),
            schema: Some(table.schema.to_string()),
            rename: None,
        }
    }
    
    fn from_str(column:&str)->Self{
        if column.contains("."){
            let splinters = column.split(".").collect::<Vec<&str>>();
            assert!(splinters.len() == 2, "There should only be 2 splinters");
            let table_split = splinters[0].to_string();
            let column_split = splinters[1].to_string();
            ColumnName{
                column:column_split.to_string(), 
                table:Some(table_split.to_string()), 
                schema:None,
                rename:None
            }
        } else {
            ColumnName{
                column:column.to_string(), 
                table:None, 
                schema:None,
                rename:None
            }
        }
    }
    
    fn rename(&self)->String{
        return format!("{}_{}", self.table.as_ref().unwrap(), self.column)
    }
    /// table name and column name
    pub fn complete_name(&self)->String{
        if self.table.is_some(){
            return format!("{}.{}", self.table.as_ref().unwrap(), self.column);
        }else{
            return self.column.to_string();
        }
    }
    /// includes the schema, table name and column name
    pub fn super_complete_name(&self)->String{
        if self.schema.is_some(){
            return format!("{}.{}", self.schema.as_ref().unwrap(), self.complete_name());
        }else{
            return self.complete_name();
        }
    }
    
}

impl PartialEq for ColumnName{
    fn eq(&self, other: &Self) -> bool{
        self.column == other.column
     }

    fn ne(&self, other: &Self) -> bool {
        self.column != other.column
    }
}


#[derive(Clone)]
#[derive(Debug)]
pub struct TableName{
    pub schema: Option<String>,
    pub name: String,
}

impl TableName{
    
    fn from_str(str: &str)->Self{
        if str.contains("."){
            let splinters = str.split(".").collect::<Vec<&str>>();
            assert!(splinters.len() == 2, "There should only be 2 splinters");
            let schema_split = splinters[0].to_string();
            let table_split = splinters[1].to_string();
            
            TableName{
                schema: Some(schema_split),
                name: table_split,
            }
            
        } else {
             TableName{
                schema: None,
                name: str.to_string(),
            }
        }
    }
    
    pub fn complete_name(&self)->String{
        match self.schema{
            Some (ref schema) => format!("{}.{}",schema, self.name),
            None => self.name.to_string()
        }
    }
}

impl PartialEq for TableName{
    fn eq(&self, other: &Self) -> bool{
        self.name == other.name && self.schema == other.schema
     }

    fn ne(&self, other: &Self) -> bool {
        self.name != other.name || self.schema != other.schema
    }
}

/// convert str, IsTable to TableName
pub trait ToTableName{
    
    fn to_table_name(&self)->TableName;
    
}

impl <'a>ToTableName for &'a str{
    
    fn to_table_name(&self)->TableName{
        TableName::from_str(self)
    }
}
impl ToTableName for Table{
    
    fn to_table_name(&self)->TableName{
        TableName{
            schema:Some(self.schema.to_string()),
            name: self.name.to_string(),
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Query{
    
    ///sql type determine which type of query to form, some fields are not applicable to other types of query
    pub sql_type:SqlType,
    
    /// whether to select the records distinct
    pub distinct:bool,
    
    pub declared_query: BTreeMap<String, Query>,

    ///fields can be functions, column sql query, and even columns
    /// TODO; merge enumerated column to this, add a builder for fields
    pub enumerated_fields:Vec<Field>,
    
    /// list of renamed columns whenever there is a conflict
    /// Vec(table, column, new_column_name)
    pub renamed_columns:BTreeMap<String, Vec<(String, String)>>,
    
    /// specify to use distinct ON set of columns 
    pub distinct_on_columns:Vec<String>,
    
    /// filter records, ~ where statement of the query
    pub filters:Vec<Filter>,
    
    /// joining multiple tables
    pub joins:Vec<Join>,
    
    /// ordering of the records via the columns specified
    pub order_by:Vec<(String, Direction)>,
    
    /// grouping columns to create an aggregate
    pub group_by: Vec<Operand>,
    
    /// having field
    pub having: Vec<Condition>,
    
    /// exclude the mention of the columns in the SQL query, useful when ignoring changes in update/insert records
    pub excluded_columns:Vec<ColumnName>,
    
    /// paging of records
    pub page:Option<usize>,
    
    /// size of a page
    pub page_size:Option<usize>,
    
    /// where the focus of values of column selection
    /// this is the table to insert to, update to delete, create, drop
    /// whe used in select, this is the 
    /// pub from_table:Option<TableName>,
    
    /// from field, where field can be a query, table, column, or function
    pub from:Option<Box<Field>>,
    
    /// The data values, used in bulk inserting, updating,
    pub values:Vec<Operand>,
    
    /// the returning clause of the query when supported,
    pub enumerated_returns: Vec<Field>,
}

impl Query{
    
    /// the default query is select
    pub fn new()->Self{
        Query{
            sql_type:SqlType::SELECT,
            distinct:false,
            declared_query: BTreeMap::new(),
            enumerated_fields: vec![],
            renamed_columns:BTreeMap::new(),
            distinct_on_columns: vec![],
            filters: vec![],
            joins: vec![],
            order_by: vec![],
            group_by: vec![],
            having: vec![],
            excluded_columns:vec![],
            page:None,
            page_size:None,
            from: None,
            values:vec![],
            enumerated_returns: vec![],
        }
    }
    
    pub fn select()->Self{
        let mut q = Query::new();
        q.sql_type = SqlType::SELECT;
        q
    }
    
    pub fn insert()->Self{
        let mut q = Query::new();
        q.sql_type = SqlType::INSERT;
        q
    }
    pub fn update()->Self{
        let mut q = Query::new();
        q.sql_type = SqlType::UPDATE;
        q
    }
    pub fn delete()->Self{
        let mut q = Query::new();
        q.sql_type = SqlType::DELETE;
        q
    }
    
    /// add DISTINCT ie: SELECT DISTINCT
    pub fn set_distinct(&mut self)->&mut Self{
        self.distinct = true;
        self
    }
    
    pub fn select_all()->Self{
        let mut q = Self::select();
        q.all();
        q
    }
    
    pub fn all(&mut self)->&mut Self{
        self.enumerate_column("*")
    }
    
    /// all enumerated columns shall be called from this
    /// any conflict of columns from some other table will be automatically renamed
    /// columns that are not conflicts from some other table,
    /// but is the other conflicting column is not explicityly enumerated will not be renamed
    /// 
    pub fn enumerate_column(&mut self, column:&str)->&mut Self{
        let column_name = ColumnName::from_str(column);
        let operand = Operand::ColumnName(column_name);
        let field = Field{operand:operand, name:None};
        self.enumerated_fields.push(field);
        self
    }
    
    
    pub fn enumerate_columns(&mut self, columns:Vec<&str>)->&mut Self{
        for c in columns{
            self.enumerate_column(c);
        }
        self
    }
    
    pub fn group_by(&mut self, columns:Vec<&str>)->&mut Self{
        for c in columns{
            let column_name = ColumnName::from_str(c);
            let operand = Operand::ColumnName(column_name);
            self.group_by.push(operand);
        }
        self
    }
    
    pub fn having(&mut self, column:&str, equality: Equality, value :&ToType)->&mut Self{
        let column_name = ColumnName::from_str(column);
        let left_operand = Operand::ColumnName(column_name);
        let cond = Condition{
            left_operand: left_operand,
            equality: equality,
            right_operand: Operand::Value(value.to_db_type())
        };
        self.having.push(cond);
        self
    }
    
    pub fn enumerate(&mut self, columns:Vec<&str>)->&mut Self{
        self.enumerate_columns(columns)
    }
    
    /// exclude columns when inserting/updating data
    /// also ignores the column when selecting records
    /// useful for manipulating thin records by excluding huge binary blobs such as images
    pub fn exclude_column(&mut self, column:&str)->&mut Self{
        let c = ColumnName::from_str(column);
        self.excluded_columns.push(c);
        self
    }
    pub fn exclude_columns(&mut self, columns:Vec<&str>)->&mut Self{
        for c in columns{
            self.exclude_column(c);
        }
        self
    }
    
    pub fn distinct_on_columns(&mut self, columns:&Vec<String>)->&mut Self{
        let columns = columns.clone();
        for c in columns{
            self.distinct_on_columns.push(c);
        }
        self
    }
    
    /// when paging multiple records
    pub fn set_page(&mut self, page:usize)->&mut Self{
        self.page = Some(page);
        self
    }
    
    /// the number of items retrieve per page
    pub fn set_page_size(&mut self, items:usize)->&mut Self{
        self.page_size = Some(items);
        self
    }

    /// the number of items retrieve per page
    pub fn limit(&mut self, limit:usize)->&mut Self{
        self.set_page_size(limit)
    }
    
    /// A more terse way to write the query
    pub fn from(&mut self, table: &ToTableName)->&mut Self{
        let table_name = table.to_table_name();
        let operand = Operand::TableName(table_name);
        let field = Field{ operand:operand, name: None};
        self.from_field(field)
    }
    
     pub fn from_table(&mut self, table:&str)->&mut Self{
        self.from(&table)
    }
    /// can not use into since it's rust .into built-in (owned)
    pub fn into_table(&mut self, table: &ToTableName)->&mut Self{
        self.sql_type = SqlType::INSERT;
        self.from(table)
    }
    
    /// if the database support CTE declareted query i.e WITH, 
    /// then this query will be declared
    /// if database doesn't support WITH queries, then this query will be 
    /// wrapped in the from_query
    /// build a builder for this
    pub fn declare_query(&mut self, query:Query, alias:&str)->&mut Self{
        self.declared_query.insert(alias.to_string(), query);
        self
    }
    
    /// a query to query from
    /// use WITH (query) t1 SELECT from t1 declaration in postgresql, sqlite
    /// use SELECT FROM (query) in oracle, mysql, others 
    /// alias of the table
    pub fn from_query(&mut self, query:Query, alias:&str)->&mut Self{
        let operand = Operand::Query(query);
        let field = Field{operand:operand, name:Some(alias.to_string())};
        self.from_field(field)
    }
    
    pub fn from_field(&mut self, field:Field)->&mut Self{
        self.from = Some(Box::new(field));
        self
    }
    
    pub fn get_from_table(&self)->Option<&TableName>{
        match self.from{
            Some(ref field) => {
                match field.operand{
                    Operand::TableName(ref table_name) => {
                        Some(table_name)
                     },
                    _ => None
                }
            },
            None => None,
        }
    }
    
    /// list down the columns of this table then add it to the enumerated list of columns
    pub fn enumerate_table_all_columns(&mut self, table: &Table)->&mut Self{
        for c in &table.columns{
            let column_name = ColumnName::from_column(c, table);
            let operand = Operand::ColumnName(column_name);
            let field = Field{operand:operand, name:None};
            self.enumerated_fields.push(field);
        }
        self
    }
    
    /// join a table on this query
    ///
    /// # Examples
    ///
    /// ```
    /// let mut q = Query::new();
    /// let join = Join{
    ///        modifier:Some(Modifier::LEFT),
    ///        join_type:Type::OUTER,
    ///        table:table,
    ///        column1:vec![column1],
    ///        column2:vec![column2]
    ///    };
    ///
    /// q.join(join);
    ///
    /// ```
    pub fn join(&mut self, join:Join)->&mut Self{
        self.joins.push(join);
        self
    }
    
    
    /// join a table on this query
    ///
    /// # Examples
    ///
    /// ```
    /// let mut q = Query::new();
    /// q.select_from_table("users");
    /// q.left_join("roles", "role_id", "role_id");
    ///
    /// ```
    
    pub fn left_join(&mut self, table:&ToTableName, column1:&str, column2:&str)->&mut Self{
        let join = Join{
            modifier:Some(Modifier::LEFT),
            join_type:JoinType::OUTER,
            table_name: table.to_table_name(),
            column1:vec![column1.to_string()],
            column2:vec![column2.to_string()]
        };
        self.join(join)
    }
    pub fn right_join(&mut self, table:&ToTableName, column1:&str, column2:&str)->&mut Self{
        let join = Join{
            modifier:Some(Modifier::RIGHT),
            join_type:JoinType::OUTER,
            table_name: table.to_table_name(),
            column1:vec![column1.to_string()],
            column2:vec![column2.to_string()]
        };
        self.join(join)
    }
    
    pub fn full_join(&mut self, table:&ToTableName, column1:&str, column2:&str)->&mut Self{
        let join = Join{
            modifier:Some(Modifier::FULL),
            join_type:JoinType::OUTER,
            table_name: table.to_table_name(),
            column1:vec![column1.to_string()],
            column2:vec![column2.to_string()]
        };
        self.join(join)
    }
    
    pub fn inner_join(&mut self, table:&ToTableName, column1:&str, column2:&str)->&mut Self{
        let join  = Join{
            modifier:None,
            join_type:JoinType::INNER,
            table_name: table.to_table_name(),
            column1:vec![column1.to_string()],
            column2:vec![column2.to_string()]
        };
        self.join(join)
    }
    
    ///ascending orderby of this column
    pub fn asc(&mut self, column:&str)->&mut Self{
        self.order_by.push((column.to_string(), Direction::ASC));
        self
    }
        ///ascending orderby of this column
    pub fn desc(&mut self, column:&str)->&mut Self{
        self.order_by.push((column.to_string(), Direction::DESC));
        self
    }
    
    
    pub fn rename(&mut self, table:&str, column:&str, new_column_name:&str)->&mut Self{
        if self.renamed_columns.get(table).is_some(){
            let mut list:&mut Vec<(String, String)> = self.renamed_columns.get_mut(table).unwrap();
            if list.contains(&(column.to_string(), new_column_name.to_string())){
                println!("This is already renamed");
            }else{
                println!("renamed {} to {}", column, new_column_name);
                list.push((column.to_string(), new_column_name.to_string()));
            }
        }
        else{
            self.renamed_columns.insert(table.to_string(), vec![(column.to_string(), new_column_name.to_string())]);
        }
        self
    }
    
    pub fn get_involved_tables(&self)->Vec<&TableName>{
        let mut tables = vec![];
        let from_table = self.get_from_table();
        if from_table.is_some(){
            tables.push(from_table.unwrap());
        }
        for j in &self.joins{
            if !tables.contains(&&j.table_name){
                tables.push(&j.table_name);
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
    pub fn finalize(&mut self)->&mut Self{
         if self.excluded_columns.is_empty() 
            && self.enumerated_fields.is_empty(){
            self.all();
        }
        let excluded_columns = &self.excluded_columns.clone();
        for i in  excluded_columns{
            self.remove_from_enumerated(&i);
        }
        self
    }
    
    fn remove_from_enumerated(&mut self, column_name: &ColumnName)->&mut Self{
        fn index_of(enumerated_fields:&Vec<Field>, column: &ColumnName)->Option<usize>{
            let mut cnt = 0;
            for field in enumerated_fields{
                match field.operand{
                    Operand::ColumnName(ref column_name) => {
                        if column_name == column{
                            return Some(cnt);
                        }
                    },
                    _ => {},
                }
                cnt += 1;
            }
            None
        }
        let index = index_of(&self.enumerated_fields, column_name);
        if index.is_some(){
            self.enumerated_fields.remove(index.unwrap());
        }
        self
    }
    
    /// return the list of enumerated columns
    /// will be used for updating records
    pub fn get_enumerated_columns(&self)->Vec<&ColumnName>{
        let mut columns = vec![];
        for field in &self.enumerated_fields{
            match field.operand{
                Operand::ColumnName(ref column_name) => {
                      columns.push(column_name);
                },
                _ => {},
            }
        }
        columns
    }
    
    
    pub fn add_filter(&mut self, filter:Filter)->&mut Self{
        self.filters.push(filter);
        self
    }
    
    pub fn filter(&mut self, column:&str, equality:Equality, value:&ToType)->&mut Self{
        self.add_filter(Filter::new(column, equality, value))
    }
    
    pub fn add_value(&mut self, value:Operand)->&mut Self{
        self.values.push(value);
        self
    }
    
    pub fn value(&mut self, value:&ToType)->&mut Self{
        let operand = Operand::Value(value.to_db_type());
        self.add_value(operand)
    }
    
    pub fn enumerate_all_table_column_as_return(&mut self, table:&Table)->&mut Self{
         for c in &table.columns{
            let column_name = ColumnName::from_column(c, table);
            let operand = Operand::ColumnName(column_name);
            let field = Field{operand: operand, name:None};
            self.enumerated_returns.push(field);
        }
         self
    }
    
     pub fn return_all(&mut self)->&mut Self{
        self.enumerate_column_as_return("*")
    }
    
    pub fn returns(&mut self, columns: Vec<&str>)->&mut Self{
        for c in columns{
            self.enumerate_column_as_return(c);
        }
        self
    }
    
    pub fn enumerate_column_as_return(&mut self, column:&str)->&mut Self{
        let column_name = ColumnName::from_str(column);
        let operand = Operand::ColumnName(column_name);
        let field = Field{operand: operand, name:None};
        self.enumerated_returns.push(field);
        self
    }
    
    /// build the query only, not executed, useful when debugging
    pub fn build(&self, db: &Database)->SqlFrag{
        db.build_query(self)
    }
    
    /// expects a return, such as select, insert/update with returning clause
    fn execute_with_return(&mut self, db: &Database)->DaoResult{
        self.finalize();
        db.execute_with_return(self)
    }
    
       /// expects a return, such as select, insert/update with returning clause
    pub fn execute_with_one_return(&mut self, db: &Database)->Dao{
        self.finalize();
        db.execute_with_one_return(self)
    }
    
    /// delete, update without caring for the return
    pub fn execute(&mut self, db: &Database)->Result<usize, String>{
        self.finalize();
        db.execute(self)
    }
    
    /// execute the query, then convert the result
    pub fn collect<T: IsDao>(&mut self, db: &Database)->Vec<T>{
        let result = self.execute_with_return(db);
        T::from_dao_result(&result)
    }
    
    /// execute the query then collect only 1 record
    /// put a limit 1 if not already
    pub fn collect_one<T: IsDao>(&mut self, db: &Database)->T{
        if self.page_size.is_none(){
            self.limit(1);
        }
        let result = self.execute_with_return(db);
        let mut dao:Vec<T> = T::from_dao_result(&result);
        assert!(dao.len() == 1, "There should only be 1 returned record");
        dao.remove(0)
    }
}
