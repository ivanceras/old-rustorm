use filter::Filter;
use query::Query;
use table::Table;
use dao::DAO;
use meta::ModelMetaData;

use database::*;

pub struct Postgres {
	value:bool,
}

impl Database for Postgres{
		fn begin(&self){}
	
	fn commit(&self){}
	
	fn rollback(&self){}
	
	fn is_transacted(&self)->bool{false}
	
	fn is_closed(&self)->bool{false} 
	
	fn is_connected(&self)->bool{false}
	
	fn close(&self){}
	
	fn is_valid(&self)->bool{false}

	fn reset(&self){}
	

	fn retrieve(&self, query:&Query)->Vec<DAO>{Vec::new()}
	
	fn update(&self, dao:DAO, model:&Table, filters:&Vec<Filter>)->DAO{DAO::new()}

	fn delete(&self, model:&Table, filters:&Vec<Filter>)->usize{0}
	
	fn empty(&self, model:&Table, forced:bool)->usize{0}

	fn write_to_blob(&self, buffer:Vec<u8>)->usize{0}
	
	fn write_to_file(&self, filename:&String){}
	
	fn get_blob(&self, oid:u64)->Vec<u8>{Vec::new()}

	fn insert(&self, dao:&DAO, meta:&ModelMetaData, model:&Table, query:&Query)->DAO{DAO::new()}
	
	fn search(&self, query:&Query, keyword:String){}

	fn correct_data_types(&self, dao_list:Vec<DAO>, model:&Table){}

	fn execute(&self, query:Query)->usize{0}
}

impl DatabaseDDL for Postgres{
	fn create_schema(&self, schema:&String){}
	
	fn drop_schema(&self, schema:&String, forced:bool){}
	
	fn create_table(&self, model:&Table){}
	
	fn rename_table(&self, schema:String, table:String, new_tablename:String){}
	
	fn drop_table(&self, schema:String, table:String, forced:bool){}
	
	fn set_foreign_constraint(&self, model:&Table){}

	fn set_primary_constraint(&self, model:&Table){}

}

impl DatabaseDev for Postgres{
	
	fn exist_table(&self, schema:String, table:String)->bool{false}
	
	fn get_sub_classes(&self, schema:String, table:String)->Vec<String>{Vec::new()}
	
	fn get_superclass(&self, schema:String, table:String)->String{String::new()}
	
	fn get_table_metadata(&self, schema:String, table:String)->Table{panic!("not yet");}
	
	fn get_all_tablenames(&self)->Vec<(String, String)>{Vec::new()}
	
	fn get_tablenames(&self, schema:Vec<String>, pattern:String)->Vec<(String, String)>{Vec::new()}
	
	fn get_table_column_comments(&self, schema:String, table:String)->Vec<(String, String)>{Vec::new()}
	
	fn get_table_comment(&self, schema:String, table:String)->String{String::new()}
	
}