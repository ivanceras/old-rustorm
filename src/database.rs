use filter::Filter;
use query::Query;
use table::{Table, Column};
use meta::ModelMetaData;
use types::Dao;
use writer::Writer;

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
	fn retrieve(&self, query:&Query)->Vec<Dao>;
	
	/// update a certain Dao object with the model definition and filter
	fn update(&self, dao:Dao, model:&Table, filters:&Vec<Filter>)->Dao;

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
	/// Insert a Dao object with the definition defined in the model argument
	/// Query when inserting a data that is coming from a Query
	/// meta is a lookup for the query building to be used
	
	fn insert(&self, dao:&Dao, meta:&ModelMetaData, model:&Table, query:&Query)->Dao;
	
	 ///
	 /// Search a set of record from the base Query that would have been returned by the base query
	 ///
	fn search(&self, query:&Query, keyword:String);

	/// Actually converting the from whatever JDBC converts the object to the correct type that we intend to be using
	fn correct_data_types(&self, dao_list:Vec<Dao>, model:&Table);

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
	
	/// applicable to later version of postgresql where there is inheritance
	fn get_table_sub_class(&self, schema:&str, table:&str)->Option<Vec<String>>;
	
	fn get_parent_table(&self, schema:&str, table:&str)->Option<String>;
	
	////
	/// Build the Table object based on the extracted meta data info from database
	/// This is queries directly from the database, so this will be costly. Only used this on initialization processes
	///
	fn get_table_metadata(&self, schema:&str, table:&str)->Table;
	
	/// get all the tables in this database
	fn get_all_tables(&self)->Vec<(String, String)>;
	
	/// get the comment of this table
	fn get_table_comment(&self, schema:&str, table:&str)->Option<String>;
	
	/// get the inherited columns of this table
	fn get_inherited_columns(&self, schema:&str, table:&str)->Vec<String>;
	
	///get the equivalent postgresql database data type to rust data type
	/// returns (module, type)
	fn get_rust_data_type(&self, db_type:&str)->(Option<Vec<String>>, String);


	///(imports, source code)
	fn to_source_code(&self, table:&Table, all_tables:&Vec<Table>)->(Vec<String>, String){
		let mut w = Writer::new();
		//imports
		let mut imports:Vec<String> = Vec::new();
		for c in &table.columns{
			let (package, _) = self.get_rust_data_type(&c.data_type);
			if package.is_some(){
				for i in package.unwrap(){
					imports.push(i);
				}
			}
		}
		imports.sort_by(|a, b| a.cmp(b));
		imports.dedup();
		
		//struct
		let struct_name = table.get_struct_name();
		w.ln();
		if table.comment.is_some(){
			w.append("///");
			w.ln();
			w.append("/// ");
			w.append(&table.comment.clone().unwrap());
			w.ln();
			w.append("///");
			w.ln();
		}
		w.append("pub struct ").append(&struct_name).appendln(" {");
		
		let mut included_columns = Vec::new();
		//primary columns
		for p in table.get_primary_columns(){
			if !included_columns.contains(&&p.name){
				w.tab();
				w.append("/// primary");
				w.ln();
				self.write_column(&mut w, p);
				included_columns.push(&p.name);
			}
		}
		//unique columns
		for u in table.get_unique_columns(){
			if !included_columns.contains(&&u.name){
				w.tab();
				w.append("/// unique");
				w.ln();
				self.write_column(&mut w, u);
				included_columns.push(&u.name);
			}
		}
		
		// uninherited columns
		for uc in &table.get_uninherited_columns(){
			if !included_columns.contains(&&uc.name){
				self.write_column(&mut w, uc);
				included_columns.push(&uc.name);
			}
		}
		
		// inherited columns
		for ic in &table.get_inherited_columns(){
			if !included_columns.contains(&&ic.name){
				self.write_column(&mut w, ic);
				included_columns.push(&ic.name);
			}
		}
		
		// foreign columns, has_one
		let foreign_tables = table.get_referred_tables(all_tables);
		for (column, ft) in foreign_tables{
			if ft.name != table.name { //do not include self referencial table
				w.tab();
				w.append("/// has one");
				w.ln();
				w.tab();
				w.append("pub ");
				let member_name = format!("{}_{}",column.name, ft.name);
				w.append(&member_name);
				w.append(":");
				w.append("Option<");
				w.append(&ft.get_struct_name());
				w.append(">");
				w.comma();
				w.ln();
			}else{
				w.tab();
				w.append("/// has one, self referential");
				w.ln();
				w.tab();
				w.append("pub ");
				let member_name = format!("{}_{}",column.name, ft.name);
				w.append(&member_name);
				w.append(":");
				w.append("Option<");
				w.append("Box<");
				w.append(&ft.get_struct_name());
				w.append(">");
				w.append(">");
				w.comma();
				w.ln();
			}
		}
		
		//extension tables
		let mut included_has_many = Vec::new();
		for ext in table.get_extension_tables(all_tables){
				w.tab();
				w.append("/// has one, extension table");
				w.ln();
				w.tab();
				w.append("pub ");
				w.append(&ext.name);
				w.append(":");
				w.append("Option<");
				w.append("Box<");//put it inside the box to get rid of illegal recursive struct
				w.append(&ext.get_struct_name());
				w.append(">");
				w.append(">");
				w.comma();
				w.ln();
				included_has_many.push(&ext.name);//put to included in hasMany to prevent it from putting it there
		}
		//indirect referring table
		for (indirect, linker_table) in table.get_indirect_referring_tables(all_tables){
				w.tab();
				w.append("/// has many, indirect referring table, derived from linker table: ");
				w.append(&linker_table.name);
				w.ln();
				w.tab();
				w.append("pub ");
				//let member_name = format!("{}_{}",col.name, idr.name);
				//w.append(&member_name);
				w.append(&indirect.name);
				w.append(":");
				w.append("Option<");
				w.append("Vec<");//put it inside the box to get rid of illegal recursive struct
				w.append(&indirect.get_struct_name());
				w.append(">");
				w.append(">");
				w.comma();
				w.ln();
				included_has_many.push(&indirect.name);//put to included in hasMany to prevent it from putting it there
		}
		
		// referring table, has_many
		let linker_tables = table.get_linker_tables(all_tables);
		for (ref_table, column) in table.get_referring_tables(all_tables){
			if !included_has_many.contains(&&ref_table.name) &&
			 !Table::in_tables(ref_table, &linker_tables) {
				w.tab();
				w.append("/// has many");
				w.ln();
				w.tab();
				w.append("pub ");
				//let member_name = format!("{}_{}_{}",ref_foreign.column, ref_table.name, column.name);
				//w.append(&member_name);
				w.append(&ref_table.name);
				w.append(":");
				w.append("Option<Vec<");
				w.append(&ref_table.get_struct_name());
				w.append(">>");
				w.comma();
				w.ln();
				included_has_many.push(&ref_table.name);
			}
		}
		
		w.append("}");
		w.ln();
		(imports, w.src)
	}
	
	fn write_column(&self, w:&mut Writer, c:&Column){
		if c.comment.is_some(){
			let comment = &c.comment.clone().unwrap();
			for split in comment.split("\n"){
				w.tab();
				w.append("/// ");
				w.append(split);
				w.ln();
			}
		}
		if c.default.is_some(){
			let default = &c.default.clone().unwrap();
			for split in default.split("\n"){
				w.tab();
				w.append("/// defaults to: ");
				w.append(split);
				w.ln();
			}
		}
		if c.not_null{
			w.tab();
			w.append("/// not nullable ");
			w.ln();
		}
		if c.is_inherited{
			w.tab();
			w.append("/// --inherited-- ");
			w.ln();
		}
		w.tab();
		w.append("pub ");
		w.append(&c.get_corrected_name());
		w.append(":");
		let (_, data_type) = self.get_rust_data_type(&c.data_type);
		if c.not_null{
			w.append(&data_type);
		}else{
			w.append("Option<");
			w.append(&data_type);
			w.append(">");
		}
		w.comma();
		w.ln();
	}
	
}