use query::{Filter, Operand};
use query::Query;
use table::Table;
use dao::Dao;
use database::{Database, DbError};
use table::IsTable;
use dao::IsDao;
use dao::ToValue;
use query::Equality;

/// A higher level API for manipulating objects in the database
/// This serves as a helper function for the query api
pub struct EntityManager<'a> {
    pub db: &'a Database,
}

impl <'a>EntityManager<'a> {

    /// Create an entity manager with the database connection provided
    pub fn new(db: &'a Database) -> Self {
        EntityManager { db: db }
    }

    /// delete records of this table
    pub fn delete(&self, table: &Table, filters: Vec<Filter>) -> usize {
        let mut query = Query::delete();
        query.FROM(table);
        for filter in filters {
            query.add_filter(filter);
        }
        match query.execute(self.db) {
            Ok(x) => x,
            Err(e) => panic!("Error deleting record {}", e),
        }
    }

    /// get all the records of this table
    pub fn get_all<T>(&self) -> Result<Vec<T>, DbError>
        where T: IsTable + IsDao
    {
        let table = T::table();
        let mut q = Query::SELECT_ALL();
        q.FROM(&table);
        q.collect(self.db)
    }

    /// get all the records of this table, but return only the columns mentioned
    pub fn get_all_only_columns<T>(&self, columns: Vec<&str>) -> Result<Vec<T>, DbError>
        where T: IsTable + IsDao
    {
        let table = T::table();
        let mut q = Query::SELECT();
        q.FROM(&table);
        q.columns(columns);
        q.collect(self.db)
    }

    /// get all the records of this table, ignoring the columns listed, mentioned the other else
    pub fn get_all_ignore_columns<T>(&self, ignore_columns: Vec<&str>) -> Result<Vec<T>, DbError>
        where T: IsTable + IsDao
    {
        let table = T::table();
        let mut q = Query::SELECT();
        q.FROM(&table);
        for c in table.columns {
            q.column(&c.name);
        }
        q.exclude_columns(ignore_columns);
        q.collect(self.db)
    }


    /// get all the distinct records of this table
    pub fn get_all_distinct<T>(&self) -> Result<Vec<T>, DbError>
        where T: IsTable + IsDao
    {
        let table = T::table();
        let mut q = Query::SELECT_ALL();
        q.distinct();
        q.FROM(&table);
        q.collect(self.db)
    }

    /// get all the records on this table which passed thru the filters
    /// any query that specified more than the parameters should use the query api
    pub fn get_all_with_filter<T>(&self, filters: Vec<Filter>) -> Result<Vec<T>, DbError>
        where T: IsTable + IsDao
    {
        let table = T::table();
        let mut q = Query::SELECT_ALL();
        q.FROM(&table);
        for f in filters {
            q.add_filter(f);
        }
        q.collect(self.db)
    }

    /// get the first records of this table that passed thru the filters
    pub fn get_one<T>(&self, filter: Filter) -> Result<T, DbError>
        where T: IsTable + IsDao
    {
        let table = T::table();
        let mut q = Query::SELECT_ALL();
        q.FROM(&table);
        q.add_filter(filter);
        q.collect_one(self.db)
    }
    ///
    /// get an exact match, the value is filter against the primary key of the table
    ///
    pub fn get_exact<T>(&self, id: &ToValue) -> Result<T, DbError>
        where T: IsTable + IsDao
    {
        let table = T::table();
        let primary = table.primary_columns();
        assert!(primary.len() == 1,
                "There should only be 1 primary column for this to work");
        let pk = primary[0].name.to_owned();

        Query::SELECT_ALL()
            .FROM(&table)
            .filter(&pk, Equality::EQ, id)
            .collect_one(self.db)
    }

/// [FIXME] The arrangement of columns are off
    pub fn insert<T>(&self, t: &T) -> Result<T, DbError>
        where T: IsTable + IsDao
    {
        let table = T::table();
        let dao = t.to_dao();
        let mut q = Query::insert();
        q.INTO(&table);
        for key in dao.values.keys() {
            q.column(key);
        }
        q.return_all();
        for key in dao.values.keys() {
            let value = dao.values.get(key);
            match value {
                Some(value) => {
                    q.add_value(value);
                }
                None => (),
            }
        }
        q.collect_one(self.db)
    }

    /// insert this record on the database, ignoring some columns
    /// which are set by the database default
    /// columns that are ignored are set by the database automatically
    pub fn insert_with_ignore_columns<T>(&self,
                                         dao: Dao,
                                         ignore_columns: Vec<&str>)
                                         -> Result<T, DbError>
        where T: IsTable + IsDao
    {
        let table = T::table();
        let mut q = Query::insert();
        q.INTO(&table);
        for key in dao.values.keys() {
            q.column(key);
        }
        q.exclude_columns(ignore_columns);
        q.return_all();
        for c in &table.columns {
            let value = dao.values.get(&c.name);
            match value {
                Some(value) => {
                    q.add_value(value);
                }
                None => (),
            }
        }
        q.collect_one(self.db)
    }

    /// insert this record on the database, explicitly setting the defaults of the columns
    /// it may produce the same result with insert_with_ignore_columns
    /// the query is different since it may mentions `created` now(),
    pub fn insert_ignore_defaulted_columns<T>(&self, _dao: Dao) -> Result<T, DbError>
        where T: IsTable + IsDao
    {
        unimplemented!()
    }

    /// this is called when there is a problem with the transaction
    pub fn reset(&self) {
        unimplemented!()
    }

    /// when there is a problem with the transaction process, this can be called
    pub fn rollback(&self) {
        unimplemented!()
    }

    /// update the Dao, return the updated Dao
    pub fn update<T>(&self, _dao: &Dao) -> Result<T, DbError>
        where T: IsTable + IsDao
    {
        unimplemented!()
    }

    /// update the Dao, return the updated Dao
    /// ignored columns will remain unchanged
    pub fn update_ignore_columns<T>(&self,
                                    _dao: &Dao,
                                    _ignore_columns: Vec<&str>)
                                    -> Result<T, DbError>
        where T: IsTable + IsDao
    {
        unimplemented!()
    }

    /// update the Dao, return the updated Dao
    /// only the columns specified, the rest is unchanged
    pub fn update_only_columns<T>(&self, _dao: &Dao, _columns: Vec<&str>) -> Result<T, DbError>
        where T: IsTable + IsDao
    {
        unimplemented!()
    }

    /// update the Dao, return the updated Dao
    /// the default columns will be reset to whatever the db's default function will come up.
    /// ie. updated column will be defaulted everytime a record is updated.
    pub fn update_ignore_defaulted_columns<T>(&self, _dao: &Dao) -> Result<T, DbError>
        where T: IsTable + IsDao
    {
        unimplemented!()
    }

    /// update the Dao with filter, return the updated Dao
    pub fn update_with_filter<T>(&self, _dao: &Dao, _filter: Vec<Filter>) -> Result<T, DbError>
        where T: IsTable + IsDao
    {
        unimplemented!()
    }

    /// whether to use insert or update
    /// insert when it is a new record
    /// update when it is an existing recor
    /// may use UPSERT in newer versions of postgres
    /// may use MERGE in oracle, mssql
    pub fn save<T>(&self, _dao: T) -> Result<T, DbError>
        where T: IsTable + IsDao
    {
        unimplemented!()
    }

    ///
    /// Search a set of record from the base Query that would have been returned by the base query
    ///
    #[allow(dead_code)]
    fn search<T>(&self, _keyword: &str) -> Result<Vec<T>, DbError>
        where T: IsTable + IsDao
    {
        unimplemented!()
    }

}
