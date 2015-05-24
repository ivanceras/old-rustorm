use filter::Filter;
use query::Query;
use table::Table;
use types::Dao;
use database::Database;

pub trait EntityManager{

    /// begin transaction
    fn begin(&self);

    /// commit transaction
    fn commit(&self);

    /// count the number of results based on this query
    fn count(&self, query:&Query)->usize;

    /// create a database table aligned to this model definition
    fn create_table(&self, model: &Table);

    /// create a schema or namespace in the database
    fn create_schema(&self, schema:String);

    /// delete records of this table
    fn delete(&self, table:String, filters:&Vec<Filter>)->usize;

    /// drop the database table
    fn drop_table(&self, schema:String, table:String, forced:bool);

    /// drop the database schema
    fn drop_schema(&self, schema:String, forced:bool);

    /// empty the database table
    fn truncate_table(&self, schema:String, table:String, forced:bool) ->usize;

    /// determine if the table exist
    fn exist_table(&self, schema: String, table:String)->bool;
    
    /// determine if the schema exist
    fn exist_schema(&self, schema: String)->bool;

    /// get all the records of this table
    fn get_all(&self, table:String)->Vec<Dao>;

    /// get all the distinct records of this table
    fn get_all_distinct(&self, table:String)->Vec<Dao>;

    /// get all the records on this table which passed thru the filters
    fn get_all_with_filter(&self, table:String, filters:Vec<Filter>)->Vec<Dao>;

    /// get the first records of this table that passed thru the filters
    fn get_one(&self, table:String, filter:Vec<Filter>)->Dao;

    /// insert this records to the database
    fn insert(&self, dao:Dao);
    
    /// this is called when there is a problem with the transaction
    fn reset(&self);

    /// retrieve records from query object
    fn retrieve(&self, query:&Query)->Vec<Dao>;

    /// when there is a problem with the transaction process, this can be called
    fn rollback(&self, );
    
    /// update the Dao, return the updated Dao
    fn update(&self, dao:&Dao)->Dao;

    /// update the Dao with filter, return the updated Dao
    fn update_with_filter(&self, dao:&Dao, filter:Vec<Filter>)->Dao;

    
}

pub struct SyncEm<'a>{
    db:&'a Database,
}

impl <'a>SyncEm<'a>{
    
    pub fn new(db:&'a Database)->Self{
        SyncEm{db:db}
    }
}

impl <'a> EntityManager for SyncEm<'a>{
    
    fn begin(&self){
        self.db.begin();
    }

    fn commit(&self){panic!("not yet")}

    fn count(&self, query:&Query)->usize{panic!("not yet")}

    fn create_table(&self, model: &Table){panic!("not yet")}

    fn create_schema(&self, schema:String){panic!("not yet")}

    fn delete(&self, table:String, filters:&Vec<Filter>)->usize{panic!("not yet")}

    fn drop_table(&self, schema:String, table:String, forced:bool){panic!("not yet")}

    fn drop_schema(&self, schema:String, forced:bool){panic!("not yet")}

    fn truncate_table(&self, schema:String, table:String, forced:bool) ->usize{panic!("not yet")}

    fn exist_table(&self, schema: String, table:String)->bool{panic!("not yet")}
    
    fn exist_schema(&self, schema: String)->bool{panic!("not yet")}

    fn get_all(&self, table:String)->Vec<Dao>{panic!("not yet")}

    fn get_all_distinct(&self, table:String)->Vec<Dao>{panic!("not yet")}

    fn get_all_with_filter(&self, table:String, filters:Vec<Filter>)->Vec<Dao>{panic!("not yet")}

    fn get_one(&self, table:String, filter: Vec<Filter>)->Dao{panic!("not yet")}

    fn insert(&self, dao:Dao){panic!("not yet")}
    
    fn reset(&self){panic!("not yet")}

    fn retrieve(&self, query:&Query)->Vec<Dao>{panic!("not yet")}

    fn rollback(&self, ){panic!("not yet")}
    
    fn update(&self, dao:&Dao)->Dao{panic!("not yet")}

    fn update_with_filter(&self, dao:&Dao, filter:Vec<Filter>)->Dao{panic!("not yet")}
}
