use filter::Filter;
use query::Query;
use table::Table;
use dao::DAO;
use meta::ModelMetaData;

pub trait Database{

/// Generic Database interface
/// This is the database interface which will should be implemented to you the specifics of each database platform
/// At least all methods on this trait should be implemented for target deployment database

	/// begin database transaction
	fn begin(&self);
	
	/// commit database transaction
	fn commit(&self);
	
	/// rollback data changes executed prior to calling the begin method	
	fn rollback(&self);
	
	/// determine if this transaction has been committed or rolledback
	fn is_transacted(&self)->bool;
	
	/// determine if the database connection closed
	fn is_closed(&self)->bool; 
	
	/// check if the database is still connected
	fn is_connected(&self)->bool;
	
	/// close the database connection
	fn close(&self);
	
	/// determine if the database connection is still valid
	fn is_valid(&self)->bool;

	/// reset the database connection
	fn reset(&self);
	

	/// execute a select statement defined by the query object
	fn retrieve(&self, query:&Query)->Vec<DAO>;
	
	/// update a certain DAO object with the model definition and filter
	fn update(&self, dao:DAO, model:&Table, filters:&Vec<Filter>)->DAO;

	/// delete records
	fn delete(&self, model:&Table, filters:&Vec<Filter>)->usize;
	
	/// empty the table
	fn empty(&self, model:&Table, forced:bool)->usize;

	/// write a binary large object to the database
	fn write_to_blob(&self, buffer:Vec<u8>)->usize;
	
	/// write the blob to a file
	fn write_to_file(&self, filename:&String);
	
	/// get the blob from the database
	fn get_blob(&self, oid:u64)->Vec<u8>;


	///
	/// Insert a DAO object with the definition defined in the model argument
	/// Query when inserting a data that is coming from a Query
	/// meta is a lookup for the query building to be used
	
	fn insert(&self, dao:&DAO, meta:&ModelMetaData, model:&Table, query:&Query)->DAO;
	
	 ///
	 /// Search a set of record from the base Query that would have been returned by the base query
	 ///
	fn search(&self, query:&Query, keyword:String);

	/// Actually converting the from whatever JDBC converts the object to the correct type that we intend to be using
	fn correct_data_types(&self, dao_list:Vec<DAO>, model:&Table);

	fn execute(&self, query:Query)->usize;

}

pub trait DatabaseDDL{
	//////////////////////////////////////////
	/// The following methods involves DDL(Data definition language) operation
	////////////////////////////////////////	

	/// create a database schema
	fn create_schema(&self, schema:&String);
	
	/// drop the database schema
	fn drop_schema(&self, schema:&String, forced:bool);
	
	/// create a database table based on the Model Definition
	fn create_table(&self, model:&Table);
	
	/// rename table
	fn rename_table(&self, schema:String, table:String, new_tablename:String);
	
	/// drop table
	fn drop_table(&self, schema:String, table:String, forced:bool);
	
	/// set the foreign key constraint of a table
	fn set_foreign_constraint(&self, model:&Table);

	/// set the primary key constraint of a table	
	fn set_primary_constraint(&self, model:&Table);
}

pub trait DatabaseDev{

////////////////////////////////////////
/// Database interface use for the development process
////////////////////////////////////////////
	
	/// determine if the table exist
	fn exist_table(&self, schema:String, table:String)->bool;
	
	/// applicable to later version of postgresql where there is inheritance
	fn get_sub_classes(&self, schema:String, table:String)->Vec<String>;
	
	fn get_superclass(&self, schema:String, table:String)->String;
	
	////
	/// Build the Table object based on the extracted meta data info from database
	/// This is queries directly from the database, so this will be costly. Only used this on initialization processes
	///
	fn get_table_metadata(&self, schema:String, table:String)->Table;
	
	/// get all the tables in this database
	fn get_all_tablenames(&self)->Vec<(String, String)>;
	
	/// get all the table names within the schema mentioned, matching the pattern string
	fn get_tablenames(&self, schema:Vec<String>, pattern:String)->Vec<(String, String)>;
	
	/// get the comments of each column of this table 
	fn get_table_column_comments(&self, schema:String, table:String)->Vec<(String, String)>;
	
	/// get the comment of this table
	fn get_table_comment(&self, schema:String, table:String)->String;
}