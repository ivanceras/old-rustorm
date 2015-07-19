use query::{Filter,Operand};
use query::Query;
use table::Table;
use dao::{Dao};
use database::Database;
use table::IsTable;
use dao::IsDao;
use dao::ToValue;
use query::Equality;

/// A higher level API for manipulating objects in the database
/// This serves as a helper function for the query api
pub struct EntityManager<'a>{
    pub db:&'a mut  Database,
}

impl <'a>EntityManager<'a>{

    /// Create an entity manager with the database connection provided
    pub fn new(db:&'a mut Database)->Self{
        EntityManager{db:db}
    }

    /// delete records of this table
    pub fn delete(&mut self, table:&Table, filters:Vec<Filter>)->usize{
        let mut query = Query::delete();
        query.from(table);
        for filter in filters{
            query.add_filter(filter);
        }
        match query.execute(self.db){
            Ok(x) => x,
            Err(e) => panic!("Error deleting record {}",e),
        }
    }

    /// get all the records of this table
    pub fn get_all<T>(&mut self)->Vec<T> where T : IsTable + IsDao{
        let table = T::table();
        let mut q = Query::select_all();
        q.from_table(&table.complete_name());
        q.collect(self.db)
    }

    /// get all the records of this table, but return only the columns mentioned
    pub fn get_all_only_columns<T>(&mut self, columns:Vec<&str>)->Vec<T> 
        where T : IsTable + IsDao{
        let table = T::table();
        let mut q = Query::select();
        q.from_table(&table.complete_name());
        q.columns(columns);
        q.collect(self.db)
    }
    
    /// get all the records of this table, ignoring the columns listed, mentioned the other else
    pub fn get_all_ignore_columns<T>(&mut self, ignore_columns:Vec<&str>)->Vec<T> 
        where T : IsTable + IsDao{
        let table = T::table();
        let mut q = Query::select();
        q.from_table(&table.complete_name());
        for c in table.columns{
            q.column(&c.name);
        }
        q.exclude_columns(ignore_columns);
        q.collect(self.db)
    }


    /// get all the distinct records of this table
    pub fn get_all_distinct<T>(&mut self)->Vec<T>
        where T : IsTable + IsDao{
        let table = T::table();
        let mut q = Query::select_all();
        q.distinct();
        q.from_table(&table.complete_name());
        q.collect(self.db)
    }

    /// get all the records on this table which passed thru the filters
    /// any query that specified more than the parameters should use the query api
    pub fn get_all_with_filter<T>(&mut self, filters:Vec<Filter>)->Vec<T> 
        where T : IsTable + IsDao{
        let table = T::table();
        let mut q = Query::select_all();
        q.from_table(&table.complete_name());
        for f in filters{
            q.add_filter(f);
        }
        q.collect(self.db)
    }

    /// get the first records of this table that passed thru the filters
    pub fn get_one<T>(&mut self, filter:Filter)->Vec<T> 
        where T : IsTable + IsDao{
        let table = T::table();
        let mut q = Query::select_all();
        q.from_table(&table.complete_name());
        q.add_filter(filter);
        q.collect(self.db)
    }
/// 
/// get an exact match, the value is filter against the primary key of the table
/// 
    pub fn get_exact<T>(&mut self, id: &ToValue)->Option<T> 
        where T : IsTable + IsDao{
        let table = T::table();
        let primary = table.primary_columns();
        assert!(primary.len() == 1, "There should only be 1 primary column for this to work");
        let pk = primary[0].name.to_string();
        
        Query::select_all()
            .from_table(&table.complete_name())
            .filter(&pk, Equality::EQ, id)
            .collect_one(self.db)
    }

    pub fn insert<T>(&mut self, dao:Dao)->Option<T>
        where T : IsTable + IsDao{
        let table = T::table();
        let mut q = Query::insert();
        q.into_table(&table.complete_name());
        for key in dao.values.keys(){
            q.column(key);
        }
        q.return_all();
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
    pub fn insert_with_ignore_columns<T>(&mut self, dao:Dao, ignore_columns:Vec<&str>)->Option<T>
        where T: IsTable + IsDao {
        let table = T::table();
        let mut q = Query::insert();
        q.into_table(&table.complete_name());
        for key in dao.values.keys(){
            q.column(key);
        }
        q.exclude_columns(ignore_columns);
        q.return_all();
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
