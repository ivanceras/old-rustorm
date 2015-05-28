use filter::Filter;
use join::Join;
use join::Modifier;
use join::JoinType;

pub enum Direction{
    ASC,
    DESC,
}

pub enum SqlType{
    //DML
    Select, 
    Insert,
    Update,
    Delete,
    Truncate,
    
    //DDL
    Create,
    Drop,
    Alter,
}

pub struct Query{
    
    //sql type determine which type of query to form, some fields are not applicable to other types of query
    pub sql_type:SqlType,
    
    //whether to use *
    pub select_all:bool,
    
    //whether to select the records distinct
    pub distinct:bool,

    //enumerate all the columns of the involved models
    pub enumerate_columns:bool,
    
    //or whether to select columns
    pub select_columns:Vec<String>,
    
    //list of renamed columns whenever there is a conflict
    // Vec(table, column, new_column_name)
    pub renamed_columns:Vec<(String, String, String)>,
    
    // to use distinct ON 
    pub distinct_on_columns:Vec<String>,
    
    //filter records
    pub filters:Vec<Filter>,
    
    //joining multiple tables
    pub joins:Vec<Join>,
    
    //order the result
    pub order_by:Vec<(String, Direction)>,
    
    //list of involved tables
    pub involved_tables:Vec<String>,
    
    //grouping columns to create an aggregate
    pub grouped_columns: Vec<String>,
    
    //exclude the mention of the columns in the SQL query, useful when ignoring changes in update/insert records
    pub excluded_columns:Vec<String>,
    
    //paging of records
    pub page:Option<usize>,
    
    //size of a page
    pub items_per_page:Option<usize>,
    
    //where the focus of values of column selection
    //this is the table to insert to, update to delete, create, drop
    pub from_table:Option<String>,
    
}

impl Query{
    
    
    //the default query is select
    pub fn new()->Self{
        Query{
            sql_type:SqlType::Select,
            select_all:false,
            distinct:false,
            enumerate_columns:true,
            select_columns:Vec::new(),
            renamed_columns:Vec::new(),
            distinct_on_columns:Vec::new(),
            filters:Vec::new(),
            joins:Vec::new(),
            order_by:Vec::new(),
            involved_tables:Vec::new(),
            grouped_columns:Vec::new(),
            excluded_columns:Vec::new(),
            page:None,
            items_per_page:None,
            from_table:None,
        }
    }
    
    pub fn select()->Self{
        let mut q = Query::new();
        q.sql_type = SqlType::Select;
        q
    }
    
    pub fn insert()->Self{
        let mut q = Query::new();
        q.sql_type = SqlType::Insert;
        q
    }
    pub fn update()->Self{
        let mut q = Query::new();
        q.sql_type = SqlType::Update;
        q
    }
    pub fn delete()->Self{
        let mut q = Query::new();
        q.sql_type = SqlType::Delete;
        q
    }
    
    pub fn create()->Self{
        let mut q = Query::new();
        q.sql_type = SqlType::Create;
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
    
    //exclude columns when inserting/updating data
    pub fn exclude_columns(&mut self, columns:&Vec<String>){
        let columns = columns.clone();
        for c in columns{
            self.excluded_columns.push(c);
        }
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
    
    pub fn from_table(&mut self, table:&String){
        self.from_table = Some(table.clone());
        self.involved_tables.push(table.clone());
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
        self.involved_tables.push(join.table.clone());
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
    
    pub fn left_join(&mut self, table:String, column1:String, column2:String){
        let join = Join{
            modifier:Some(Modifier::LEFT),
            join_type:JoinType::OUTER,
            table:table,
            column1:vec![column1],
            column2:vec![column2]
        };
        self.join(join);
    }
    pub fn right_join(&mut self, table:String, column1:String, column2:String){
        let join = Join{
            modifier:Some(Modifier::RIGHT),
            join_type:JoinType::OUTER,
            table:table,
            column1:vec![column1],
            column2:vec![column2]
        };
        self.join(join);
    }
    
    pub fn full_join(&mut self, table:String, column1:String, column2:String){
        let join = Join{
            modifier:Some(Modifier::FULL),
            join_type:JoinType::OUTER,
            table:table,
            column1:vec![column1],
            column2:vec![column2]
        };
        self.join(join);
    }
    
    pub fn inner_join(&mut self, table:String, column1:String, column2:String){
        let join  = Join{
            modifier:None,
            join_type:JoinType::INNER,
            table:table,
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
    
    
    pub fn rename(&mut self, table:String, column:String, new_column_name:String){
        self.renamed_columns.push((table, column, new_column_name));
    }
}