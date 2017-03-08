use database::{Database, DbError};
use query::IsTable;
use dao::IsDao;
use query::Select;
use query::Filter;
use query::Delete;
use query::Insert;
use query::Update;
use dao::Value;

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

    /// delete records of the table that passes the provided filter
    pub fn delete<T>(&self, filter: &Filter) -> Result<usize,DbError> 
        where T:IsTable{
        let table_name = T::table_name();
        let mut query = Delete::from(&table_name);
        query.add_filter(filter);
        query.execute(self.db)
    }

    /// get all the records of this table
    pub fn get_all<T>(&self) -> Result<Vec<T>, DbError>
        where T: IsTable + IsDao {
        let table = T::table_name();
        let mut q = Select::all();
        q.from(&table);
        q.collect(self.db)
    }



    /// get all the records on this table which passed thru the filters
    /// any query that specified more than the parameters should use the query api
    pub fn get_all_with_filter<T>(&self, filter: &Filter) -> Result<Vec<T>, DbError>
        where T: IsTable + IsDao {
        let table = T::table_name();
        let mut q = Select::all();
        q.from(&table);
        q.add_filter(filter);
        q.collect(self.db)
    }

    /// get the first records of this table that passed thru the filter
    pub fn get_one<T>(&self, filter: &Filter) -> Result<T, DbError>
        where T: IsTable + IsDao {
        let table = T::table_name();
        let mut q = Select::all();
        q.from(&table);
        q.add_filter(filter);
        q.collect_one(self.db)
    }


    /// insert a record into the database
    pub fn insert<T,D>(&self, t: &T) -> Result<D, DbError>
        where T: IsTable + IsDao, D: IsDao
    {
        let table = T::table_name();
        let dao = t.to_dao();
        let mut q = Insert::into(&table);
        for c in &table.columns {
            q.column(&c.column);
        }
        q.return_all();
        for c in &table.columns {
            let value:Option<&Value> = dao.get(&c.column);
            match value {
                Some(value) => {
                    q.value(value);
                }
                None => (),
            }
        }
        q.insert(self.db)
    }

    /// starts a database transaction
    /// the next succedding function calls will be
    /// wrapped in a transaction and will not effect the database
    /// until the commit at the end is called
    pub fn begin(&self) {
        self.db.begin()
    }

    /// commits a database transaction
    pub fn commit(&self) {
        self.db.commit()
    }
    /// when there is a problem with the transaction process, this can be called
    pub fn rollback(&self) {
        self.db.rollback()
    }



    /// update the Dao with filter, return the updated Dao
    pub fn update_with_filter<T,D>(&self, t: &T, filter: Filter) -> Result<D, DbError>
        where T: IsTable + IsDao, D: IsTable + IsDao {

        let table = T::table_name();
        let dao = t.to_dao();

        let mut query = Update::table(&table);
        query.columns(&table.columns);
        for c in &table.columns{
            let v = dao.get(&c.column);
            match v{
                Some(v) => {
                    query.value(v);
                },
                None => (),
            }
        }
        query.add_filter(&filter);
        query.return_all();
        query.update(self.db)
    }

}
