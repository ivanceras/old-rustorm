use query::{Filter,Operand};
use query::Query;
use table::Table;
use dao::{Dao};
use database::Database;
use table::IsTable;
use dao::IsDao;
use dao::ToType;
use query::Equality;

/// A higher level API for manipulating objects in the database
pub struct EntityManager<'a>{
    pub db:&'a Database,
}

impl <'a>EntityManager<'a>{

    /// Create an entity manager with the database connection provided
    pub fn new(db:&'a Database)->Self{
        EntityManager{db:db}
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
        let mut query = Query::delete();
        for filter in filters{
            let f = filter.clone();
            query.add_filter(f);
        }
        match query.execute(self.db){
            Ok(x) => x,
            Err(e) => panic!("Error deleting record {}",e),
        }
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
    pub fn get_all<T>(&self)->Vec<T> where T : IsTable + IsDao{
        let table = T::table();
        let mut q = Query::select();
        q.enumerate_table_all_columns(&table);
        q.from_table(&table);
        q.collect(self.db)
    }

    /// get all the records of this table, but return only the columns mentioned
    pub fn get_all_only_columns<T>(&self, columns:Vec<&str>)->Vec<T> 
        where T : IsTable + IsDao{
        let table = T::table();
        let mut q = Query::select();
        q.from_table(&table);
        q.enumerate_columns(columns);
        q.collect(self.db)
    }
    
    /// get all the records of this table, ignoring the columns listed, mentioned the other else
    pub fn get_all_ignore_columns<T>(&self, ignore_columns:Vec<&str>)->Vec<T> 
        where T : IsTable + IsDao{
        let table = T::table();
        let mut q = Query::select();
        q.from_table(&table);
        q.enumerate_table_all_columns(&table);
        q.exclude_columns(ignore_columns);
        q.collect(self.db)
    }


    /// get all the distinct records of this table
    pub fn get_all_distinct(&self, table:&Table)->Vec<Dao>{
        panic!("not yet")
    }

    /// get all the records on this table which passed thru the filters
    /// any query that specified more than the parameters should use the query api
    pub fn get_all_with_filter<T>(&self, table:&Table, filters:Vec<Filter>)->Vec<T> 
        where T : IsTable + IsDao{
        let table = T::table();
        let mut q = Query::select();
        q.from_table(&table);
        q.enumerate_table_all_columns(&table);
        for f in filters{
            q.add_filter(f);
        }
        q.collect(self.db)
    }

    /// get the first records of this table that passed thru the filters
    pub fn get_one<T>(&self, filter:Filter)->Vec<T> 
        where T : IsTable + IsDao{
        let table = T::table();
        let mut q = Query::select();
        q.from_table(&table);
        q.enumerate_table_all_columns(&table);
        q.add_filter(filter);
        q.collect(self.db)
    }
/// 
/// get an exact match, the value is filter against the primary key of the table
/// # Examples
/// ```rust,no_run
/// extern crate rustorm;
/// extern crate uuid;
/// extern crate chrono;
/// extern crate rustc_serialize;
/// 
/// use rustorm::db::postgres::Postgres;
/// use uuid::Uuid;
/// 
/// use rustorm::em::EntityManager;
/// use gen::bazaar::Product;
/// 
/// mod gen;
///  
/// 
/// fn main(){
///     let pg= Postgres::connect_with_url("postgres://postgres:p0stgr3s@localhost/bazaar_v6").unwrap();
///     let em = EntityManager::new(&pg);
///     let pid = Uuid::parse_str("6db712e6-cc50-4c3a-8269-451c98ace5ad").unwrap();
///     let prod: Product = em.get_exact(&pid);
///     println!("{}  {}  {:?}", prod.product_id, prod.name.unwrap(), prod.description);
/// }
/// ```
/// 
    pub fn get_exact<T>(&self, id: &ToType)->T 
        where T : IsTable + IsDao{
        let table = T::table();
        let primary = table.primary_columns();
        assert!(primary.len() == 1, "There should only be 1 primary column for this to work");
        let pk = primary[0].name.to_string();
        
        let mut q = Query::select();
        q.from_table(&table);
        q.enumerate_table_all_columns(&table);
        q.filter(&pk, Equality::EQ, id);
        q.collect_one(self.db)
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
    /// let pg = Postgres::with_connection("postgres://postgres:p0stgr3s@localhost/bazaar_v6");
    /// match pg{
    ///     Ok(pg) => {
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
    pub fn insert<T>(&self, dao:Dao)->T
        where T : IsTable + IsDao{
        let table = T::table();
        let mut q = Query::insert();
        q.into_table(&table);
        for key in dao.values.keys(){
            q.enumerate_column(key);
        }
        q.enumerate_all_table_column_as_return(&table);
        for c in &table.columns{
            let value = dao.values.get(&c.name);
            match value{
                Some(value) => {
                    q.add_value(Operand::Value(value.clone()));
                }
                None => (),
            };
        }
        q.collect_one(self.db)
    }

    /// insert this record on the database, ignoring some columns
    /// which are set by the database default
    /// columns that are ignored are set by the database automatically
    pub fn insert_with_ignore_columns<T>(&self, dao:Dao, ignore_columns:Vec<&str>)->T
        where T: IsTable + IsDao {
        let table = T::table();
        let mut q = Query::insert();
        q.into_table(&table);
        for key in dao.values.keys(){
            q.enumerate_column(key);
        }
        q.exclude_columns(ignore_columns);
        q.enumerate_all_table_column_as_return(&table);
        for c in &table.columns{
            let value = dao.values.get(&c.name);
            match value{
                Some(value) => {
                    q.add_value(Operand::Value(value.clone()));
                }
                None => (),
            };
        }
        q.collect_one(self.db)
    }

    /// insert this record on the database, explicitly setting the defaults of the columns
    /// it may produce the same result with insert_with_ignore_columns
    /// the query is different since it may mentions `created` now(),
    pub fn insert_ignore_defaulted_columns<T>(&self, dao:Dao)->T
        where T: IsTable + IsDao {
        panic!("not yet")
    }

    /// this is called when there is a problem with the transaction
    pub fn reset(&self){
        panic!("not yet")
    }

    /// when there is a problem with the transaction process, this can be called
    pub fn rollback(&self){
        panic!("not yet")
    }

    /// update the Dao, return the updated Dao
    pub fn update<T>(&self, dao:&Dao)->T
        where T: IsTable + IsDao {
        panic!("not yet")
    }

    /// update the Dao, return the updated Dao
    /// ignored columns will remain unchanged
    pub fn update_ignore_columns<T>(&self, dao:&Dao, ignore_columns:Vec<&str>)->T
        where T: IsTable + IsDao {
        panic!("not yet")
    }

    /// update the Dao, return the updated Dao
    /// only the columns specified, the rest is unchanged
    pub fn update_only_columns<T>(&self, dao:&Dao, columns:Vec<&str>)->T
        where T: IsTable + IsDao {
        panic!("not yet")
    }

    /// update the Dao, return the updated Dao
    /// the default columns will be reset to whatever the db's default function will come up.
    /// ie. updated column will be defaulted everytime a record is updated.
    pub fn update_ignore_defaulted_columns<T>(&self, dao:&Dao)->T
        where T: IsTable + IsDao {
        panic!("not yet")
    }

    /// update the Dao with filter, return the updated Dao
    pub fn update_with_filter<T>(&self, dao:&Dao, filter:Vec<Filter>)->T
        where T: IsTable + IsDao {
        panic!("not yet")
    }
    
    /// whether to use insert or update
    /// insert when it is a new record
    /// update when it is an existing recor
    /// may use UPSERT in newer versions of postgres
    /// may use MERGE in oracle, mssql
    pub fn save<T>(&self, dao:T)->T where T : IsTable + IsDao{
        panic!("not yet");
    }
     ///
     /// Search a set of record from the base Query that would have been returned by the base query
     ///
    fn search<T>(&self, keyword:&str)->Vec<T>
        where T: IsTable + IsDao {
        panic!("not yet");
    }

}
