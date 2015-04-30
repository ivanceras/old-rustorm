use filter::Filter;
use query::Query;
use table::Table;
use dao::DAO;

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
	fn get_all(&self, table:String)->Vec<DAO>;

	/// get all the distinct records of this table
	fn get_all_distinct(&self, table:String)->Vec<DAO>;

	/// get all the records on this table which passed thru the filters
	fn get_all_with_filter(&self, table:String, filters:Vec<Filter>)->Vec<DAO>;

	/// get the first records of this table that passed thru the filters
	fn get_one(&self, table:String, Vec<Filter>)->DAO;

	/// insert this records to the database
	fn insert(&self, dao:DAO);
	
	/// this is called when there is a problem with the transaction
	fn reset(&self);

	/// retrieve records from query object
	fn retrieve(&self, query:&Query)->Vec<DAO>;

	/// when there is a problem with the transaction process, this can be called
	fn rollback(&self, );
	
	/// update the DAO, return the updated DAO
	fn update(&self, dao:&DAO)->DAO;

	/// update the DAO with filter, return the updated DAO
	fn update_with_filter(&self, dao:&DAO, filter:Vec<Filter>)->DAO;

	
}
