use filter::Filter;
use query::Query;
use model::ModelDef;
use dao::DAO;
use meta::ModelMetaData;
use sql::SQL;

/// Generic Database interface
/// This is the database interface which will should be implemented to you the specifics of each database platform

pub trait Database{

	fn begin(&self);
	
	fn commit(&self);
	
	fn rollback(&self);
	
	fn is_transacted(&self)->bool;
	
	fn is_closed(&self)->usize; 
	
	fn is_valid(&self)->usize;

	fn reset(&self)->usize;
	
	fn is_connected(&self)->bool;
	
	fn close(&self);

	fn select(&self, meta: &ModelMetaData , query:&Query)->Vec<DAO>;
	
	/// Executes a SQL statement and set which columns have been renamed to
	fn select_with_renamed_columns(&self, sql:&SQL, renamedColumns:Vec<(String, Vec<String>)>)->Vec<DAO>;
	
	fn select_sql<T>(&self, sql:String, parameters:&Vec<T>)->Vec<DAO>;

	fn update_sql<T>(&self, sql:String, parameters:&Vec<T>)->Vec<DAO>;

	fn update(&self, dao:DAO, model:&ModelDef, filters:&Vec<Filter>)->DAO;

	fn delete(&self, model:&ModelDef, filters:&Vec<Filter>)->usize;
	
	fn empty(&self, model:&ModelDef, forced:bool)->usize;

	fn write_to_blob(&self, buffer:Vec<u8>)->u64;
	
	fn write_to_file(&self, filename:&String);
	
	fn get_blob(&self, oid:u64)->Vec<u8>;


	///
	/// Colnverts the Query object into a SQL object that will be readily executed by the Database platform
	///
	fn build_sql(&self, meta: &ModelMetaData ,query:&Query, use_cursor:bool)->SQL;
	

	
	///
	/// Insert a DAO object with the definition defined in the model argument
	/// Query when inserting a data that is coming from a Query
	/// meta is a lookup for the query building to be used
	
	fn insert(&self, dao:&DAO, meta:&ModelMetaData, model:&ModelDef, query:&Query)->DAO;
	
	 ///
	 /// Search a set of record from the base Query that would have been returned by the base query
	 ///
	fn search(&self, query:&Query, keyword:String);

	/// Actually converting the from whatever JDBC converts the object to the correct type that we intend to be using
	fn correct_data_types(&self, dao_list:Vec<DAO>, model:&ModelDef);

	fn execute(&self, sql:&SQL)->usize;

}

/// This methods involves DDL(Data definition language) operation
pub trait DatabaseDDL{
	
	fn create_schema(&self, schema:&String);
	
	fn drop_schema(&self, schema:&String, forced:bool);
	
	fn create_table(&self, model:&ModelDef);
	
	fn rename_table(&self, schema:String, table:String, new_tablename:String);
	
	fn drop_table(&self, schema:String, table:String, forced:bool);
	
	fn set_foreign_constraint(&self, model:&ModelDef);
	
	fn set_primary_constraint(&self, model:&ModelDef);
}

/// Database interface use for the development process
pub trait DatabaseDev{
	
	fn exist_table(&self, schema:String, table:String)->bool;
	
	/// applicable to later version of postgresql where there is inheritance
	fn get_sub_classes(&self, schema:String, table:String)->Vec<String>;
	
	fn get_superclass(&self, schema:String, table:String)->String;
	
	////
	/// Build the ModelDef object based on the extracted meta data info from database
	/// This is queries directly from the database, so this will be costly. Only used this on initialization processes
	///
	fn get_table_metadata(&self, schema:String, table:String)->ModelDef;
	
	/// get all the tables in this database
	fn get_all_tablenames(&self)->Vec<(String, String)>;
	
	/// get all the table names within the schema mentioned, matching the pattern string
	fn get_tablenames(&self, schema:Vec<String>, pattern:String)->Vec<(String, String)>;
	
	/// get the comments of each column of this table 
	fn get_table_column_comments(&self, schema:String, table:String)->Vec<(String, String)>;
	
	/// get the comment of this table
	fn get_table_comment(&self, schema:String, table:String)->String;
}