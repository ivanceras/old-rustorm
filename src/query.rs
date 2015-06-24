use filter::Filter;
use dao::Dao;
use table::{Table, Column};
use std::collections::BTreeMap;


pub enum JoinType{
    CROSS,
    INNER,
    OUTER,
}

pub enum Modifier{
    LEFT,
    RIGHT,
    FULL,
}


pub struct Join{
    pub modifier:Option<Modifier>,
    pub join_type:JoinType,
    pub table_name:TableName,
    pub column1:Vec<String>,
    pub column2:Vec<String>
}

pub enum Direction{
    ASC,
    DESC,
}

/// Could have been SqlAction
pub enum SqlType{
    //DML
    SELECT,
    INSERT,
    UPDATE,
    DELETE,
}

#[derive(Clone)]
pub struct ColumnName{
    pub column:String,
    pub table:String,
    ////optional schema, if ever there are same tables resideing in  different schema/namespace
    pub schema:Option<String>,
    /// as rename
    pub rename:Option<String>
}

impl ColumnName{

    fn from_column(column:&Column, table:&Table)->Self{
        ColumnName{
            column: column.name.to_string(),
            table: table.name.to_string(),
            schema: Some(table.schema.to_string()),
            rename: None,
        }
    }
    
    fn rename(&self)->String{
        return format!("{}_{}", self.table, self.column)
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

/*
impl PartialOrd for ColumnName{
    
}
*/

#[derive(Clone)]
pub struct TableName{
    pub schema: String,
    pub name: String,
    pub column_names: Vec<ColumnName>,
}

impl TableName{
    
    fn from_table(table:&Table)->Self{
        TableName{
            schema:table.schema.to_string(),
            name: table.name.to_string(),
            column_names:vec![],
        }
    }

    pub fn complete_name(&self)->String{
        format!("{}.{}",self.schema, self.name)
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

pub struct Query{
    
    ///sql type determine which type of query to form, some fields are not applicable to other types of query
    pub sql_type:SqlType,
    
    ///whether to use *
    pub select_all:bool,
    
    /// whether to select the records distinct
    pub distinct:bool,

    /// enumerate all the columns of the table involved in the query 
    pub enumerate_columns:bool,
    
    ///or whether to select columns
    pub enumerated_columns:Vec<ColumnName>,
    
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
    pub grouped_columns: Vec<String>,
    
    /// exclude the mention of the columns in the SQL query, useful when ignoring changes in update/insert records
    pub excluded_columns:Vec<ColumnName>,
    
    /// paging of records
    pub page:Option<usize>,
    
    /// size of a page
    pub items_per_page:Option<usize>,
    
    /// where the focus of values of column selection
    /// this is the table to insert to, update to delete, create, drop
    /// whe used in select, this is the 
    pub from_table:Option<TableName>,
    
    /// The data values, used in bulk inserting, updating,
    pub dao:Vec<Dao>,
    
}

impl Query{
    
    
    //the default query is select
    pub fn new()->Self{
        Query{
            sql_type:SqlType::SELECT,
            select_all:false,
            distinct:false,
            enumerate_columns:true,
            enumerated_columns:Vec::new(),
            renamed_columns:BTreeMap::new(),
            distinct_on_columns:Vec::new(),
            filters:Vec::new(),
            joins:Vec::new(),
            order_by:Vec::new(),
            grouped_columns:Vec::new(),
            excluded_columns:Vec::new(),
            page:None,
            items_per_page:None,
            from_table:None,
            dao:vec![],
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
    
    //add DISTINCT ie: SELECT DISTINCT
    pub fn set_distinct(&mut self){
        self.distinct = true;
    }
    
    //enumerate all the columns involved in the query
    pub fn enumerate(&mut self){
        self.select_all = false;
        self.enumerate_columns = true;
    }
    
    /// all enumerated columns shall be called from this
    /// any conflict of columns from some other table will be automatically renamed
    /// columns that are not conflicts from some other table,
    /// but is the other conflicting column is not explicityly enumerated will not be renamed
    /// 
    pub fn enumerate_column(&mut self, table:&String, column:&String){
        let c = ColumnName{
            column:column.clone(), 
            table:table.clone(), 
            schema:None,
            rename:None
        };
        self.enumerated_columns.push(c);
    }
    
    /// exclude columns when inserting/updating data
    /// [FIXME] ?? remove from the enumerated_columns
    /// can this be called before the mentioned of the enumerated column?
    /// else these needs to be stored and have a final list of columns
    /// that is mentioned in the query
    pub fn exclude_column(&mut self, table:&Table, column:&String){
        let c = ColumnName{
                column:column.clone(),
                table: table.name.to_string(),
                schema: Some(table.schema.to_string()),
                rename:None,
            };
        self.excluded_columns.push(c);
    }
    
    pub fn distinct_on_columns(&mut self, columns:&Vec<String>){
        let columns = columns.clone();
        for c in columns{
            self.distinct_on_columns.push(c);
        }
    }
    
    
    pub fn set_page(&mut self, page:usize){
        self.page = Some(page);
    }
    
    pub fn set_items_per_page(&mut self, items:usize){
        self.items_per_page = Some(items);
    }

    pub fn from_table(&mut self, table:&Table){
        self.from_table = Some(TableName::from_table(table));
    }
    
    /// a query to query from
    /// use WITH (query) t1 SELECT from t1 declaration in postgresql, sqlite
    /// use SELECT FROM (query) in oracle, mysql, others 
    pub fn from_query(&mut self, query:&Query){
        
    }
    
    /// list down the columns of this table then add it to the enumerated list of columns
    pub fn enumerate_columns(&mut self, table: &Table){
        for c in &table.columns{
            self.enumerated_columns.push(ColumnName::from_column(c, table));
        }
    }
    
    /// join a table on this query
    ///
    // # Examples
    //
    // ```
    // let mut q = Query::new();
    // let join = Join{
    //        modifier:Some(Modifier::LEFT),
    //        join_type:Type::OUTER,
    //        table:table,
    //        column1:vec![column1],
    //        column2:vec![column2]
    //    };
    //
    // q.join(join);
    //
    // ```
    pub fn join(&mut self, join:Join){
        self.joins.push(join);
    }
    
    
    /// join a table on this query
    ///
    // # Examples
    //
    // ```
    // let mut q = Query::new();
    // q.select_from_table("users");
    // q.left_join("roles", "role_id", "role_id");
    //
    // ```
    
    pub fn left_join(&mut self, table:&Table, column1:String, column2:String){
        let join = Join{
            modifier:Some(Modifier::LEFT),
            join_type:JoinType::OUTER,
            table_name: TableName::from_table(table),
            column1:vec![column1],
            column2:vec![column2]
        };
        self.join(join);
    }
    pub fn right_join(&mut self, table:&Table, column1:String, column2:String){
        let join = Join{
            modifier:Some(Modifier::RIGHT),
            join_type:JoinType::OUTER,
            table_name: TableName::from_table(table),
            column1:vec![column1],
            column2:vec![column2]
        };
        self.join(join);
    }
    
    pub fn full_join(&mut self, table:&Table, column1:String, column2:String){
        let join = Join{
            modifier:Some(Modifier::FULL),
            join_type:JoinType::OUTER,
            table_name: TableName::from_table(table),
            column1:vec![column1],
            column2:vec![column2]
        };
        self.join(join);
    }
    
    pub fn inner_join(&mut self, table:&Table, column1:String, column2:String){
        let join  = Join{
            modifier:None,
            join_type:JoinType::INNER,
            table_name: TableName::from_table(table),
            column1:vec![column1],
            column2:vec![column2]
        };
        self.join(join);
    }
    
    ///ascending orderby of this column
    pub fn asc(&mut self, column:String){
        self.order_by.push((column, Direction::ASC));
    }
        ///ascending orderby of this column
    pub fn desc(&mut self, column:String){
        self.order_by.push((column, Direction::DESC));
    }
    
    
    pub fn rename(&mut self, table:&str, column:&str, new_column_name:&str){
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
    }
    
    pub fn get_involved_tables(&self)->Vec<TableName>{
        let mut tables = vec![];
        if self.from_table.is_some(){
            let from_table = self.from_table.clone().unwrap();
            tables.push(from_table);
        }
        for j in &self.joins{
            let join_table = j.table_name.clone();
            if !tables.contains(&join_table){
                tables.push(join_table);
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
    pub fn finalize(&mut self){
        
        /// function inside function
        fn index_of(enumerated_column:&Vec<ColumnName>, column:&ColumnName)->Option<usize>{
            let mut cnt = 0;
            for c in enumerated_column{
                if c == column{
                    return Some(cnt);
                }
                cnt += 1;
            }
            None
        }
        
        for i in &self.excluded_columns{
            if self.enumerated_columns.contains(i){
                println!("removing {}", i.column);
                let index = index_of(&self.enumerated_columns, i);
                self.enumerated_columns.remove(index.unwrap());
            }
        }
    }
    
    
    
    pub fn filter(&mut self, filter:Filter){
        self.filters.push(filter);
    }
}
