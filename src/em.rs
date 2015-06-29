use query::{Filter,Operand};
use query::Query;
use table::Table;
use dao::{Dao, DaoResult};
use database::Database;

/// A higher level API for manipulating objects in the database
pub struct EntityManager<'a>{
    pub db:&'a Database,
}

impl <'a>EntityManager<'a>{

    /// Create an entity manager with the database connection provided
    pub fn new(db:&'a Database)->Self{
        EntityManager{db:db}
    }

    /// begin transaction
    pub fn begin(&self){
        panic!("not yet")
    }

    /// commit transaction
    pub fn commit(&self){
        panic!("not yet")
    }

    /// count the number of results of this query
    pub fn count(&self, query:&Query)->usize{
        panic!("not yet")
    }
    /// create a database table aligned to this table definition
    pub fn create_table(&self, table: &Table){
        panic!("not yet")
    }
    /// create a schema or namespace in the database
    pub fn create_schema(&self, schema: &str){
        panic!("not yet")
    }

    /// delete records of this table
    pub fn delete(&self, table:&Table, filters:&Vec<Filter>)->usize{
        panic!("not yet")
    }

    /// drop the database table
    pub fn drop_table(&self, table:&Table){
        panic!("not yet")
    }

    /// drop the database schema
    pub fn drop_schema(&self, schema:&str){
        panic!("not yet")
    }

    /// empty the database table
    pub fn truncate_table(&self, table:&Table) ->usize{
        panic!("not yet")
    }

    /// determine if the table exist
    pub fn exist_table(&self, table:&Table)->bool{
        panic!("not yet")
    }

    /// determine if the schema exist
    pub fn exist_schema(&self, schema: &str)->bool{
        panic!("not yet")
    }

    /// get all the records of this table
    pub fn get_all(&self, table:&Table)->DaoResult{
        println!("Getting all values from table: {}",table.name);
        let mut q = Query::select();
        q.from_table(table);
        q.enumerate_table_all_columns(table);
        self.retrieve(&mut q)
    }

    /// get all the records of this table, but return only the columns mentioned
    pub fn get_all_only_columns(&self, table:&Table, columns:Vec<&str>)->DaoResult{
        println!("Getting all values from table: {}",table.name);
        let mut q = Query::select();
        q.from_table(&table);
        for c in &columns{
            q.enumerate_table_column(&table.name, &c.to_string());
        }
        self.retrieve(&mut q)
    }
    
    /// get all the records of this table, ignoring the columns listed, mentioned the other else
    pub fn get_all_ignore_columns(&self, table:&Table, ignore_columns:Vec<&str>)->DaoResult{
        println!("Getting all values from table: {}",table.name);
        let mut q = Query::select();
        q.from_table(&table);
        q.enumerate_table_all_columns(&table);
        for c in &ignore_columns{
            q.exclude_column(table, &c.to_string());
        }
        self.retrieve(&mut q)
    }


    /// get all the distinct records of this table
    pub fn get_all_distinct(&self, table:&Table)->Vec<Dao>{
        panic!("not yet")
    }

    /// get all the records on this table which passed thru the filters
    /// any query that specified more than the parameters should use the query api
    pub fn get_all_with_filter(&self, table:&Table, filters:Vec<Filter>)->DaoResult{
         println!("Getting all values from table: {}",table.name);
        let mut q = Query::select();
        q.from_table(table);
        q.enumerate_table_all_columns(table);
        for f in filters{
            q.add_filter(f);
        }
        self.retrieve(&mut q)
    }

    /// get the first records of this table that passed thru the filters
    pub fn get_one(&self, table:&Table, filter:Vec<Filter>)->Dao{
        panic!("not yet")
    }

    /// insert this records to the database, return the inserted dao with
    /// values from default columns included
    /// # Example
    /// 
    /// ```
    /// extern crate bazaar;
    /// extern crate rustorm;
    /// 
    /// use rustorm::em::EntityManager;
    /// use rustorm::db::Postgres;
    /// use rustorm::dao::Dao;
    /// use bazaar::gen::bazaar::Product;
    /// fn main(){
    /// let pg:Result<Postgres,&str> = Postgres::new("postgres://postgres:p0stgr3s@localhost/bazaar_v6");
    /// match pg{
    /// 	Ok(pg) => {
    ///         let em = EntityManager::new(&pg);
    ///         let mut dao = Dao::new();
    ///         dao.set("name", &"inserting 1 records");
    ///         dao.set("description", &"testing insert 1 record to product");
    ///         let dao = em.insert(&Product::table(), dao);
    ///         let prod = Product::from_dao(&dao);
    ///         println!("created: {}", prod.created);
    ///     }
    ///     Err(error) =>{
    ///         println!("{}",error);
    ///     }
    /// }
    /// }
    /// ```
    pub fn insert(&self, table:&Table, dao:Dao)->Dao{
        let mut q = Query::insert();
        q.into_table(table);
        for key in dao.values.keys(){
            q.enumerate_column(key);
        }
        q.enumerate_all_table_column_as_return(table);
        for c in &table.columns{
            let value = dao.values.get(&c.name);
            match value{
                Some(value) => {
                    q.add_value(Operand::Value(value.clone()));
                }
                None => (),
            };
        }
        self.db.insert(&q)
    }

    /// insert this record on the database, ignoring some columns
    /// which are set by the database default
    /// columns that are ignored are set by the database automatically
    pub fn insert_with_ignore_columns(&self, dao:Dao, ignore_columns:Vec<&str>)->Dao{
        panic!("not yet")
    }

    /// insert this record on the database, explicitly setting the defaults of the columns
    /// it may produce the same result with insert_with_ignore_columns
    /// the query is different since it may mentions `created` now(),
    pub fn insert_set_default_columns(&self, dao:Dao, default_columns:Vec<&str>)->Dao{
        panic!("not yet")
    }

    /// this is called when there is a problem with the transaction
    pub fn reset(&self){
        panic!("not yet")
    }

    /// retrieve records from query object
    pub fn retrieve(&self, query:&mut Query)->DaoResult{
        query.finalize();
        self.db.select(query)
    }

    /// when there is a problem with the transaction process, this can be called
    pub fn rollback(&self){
        panic!("not yet")
    }

    /// update the Dao, return the updated Dao
    pub fn update(&self, dao:&Dao)->Dao{
        panic!("not yet")
    }

    /// update the Dao, return the updated Dao
    /// ignored columns will remain unchanged
    pub fn update_with_ignore_columns(&self, dao:&Dao, ignore_columns:Vec<&str>)->Dao{
        panic!("not yet")
    }

    /// update the Dao, return the updated Dao
    /// only the columns specified, the rest is unchanged
    pub fn update_with_only_columns(&self, dao:&Dao, columns:Vec<&str>)->Dao{
        panic!("not yet")
    }

    /// update the Dao, return the updated Dao
    /// the default columns will be reset to whatever the db's default function will come up.
    /// ie. updated column will be defaulted everytime a record is updated.
    pub fn update_set_default_columns(&self, dao:&Dao, set_default_columns:Vec<&str>)->Dao{
        panic!("not yet")
    }

    /// update the Dao with filter, return the updated Dao
    pub fn update_with_filter(&self, dao:&Dao, filter:Vec<Filter>)->Dao{
        panic!("not yet")
    }

     ///
     /// Search a set of record from the base Query that would have been returned by the base query
     ///
    fn search(&self, query:&Query, keyword:&str)->Vec<Dao>{
        panic!("not yet");
    }

}
