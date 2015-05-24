use filter::Filter;
use query::Query;
use table::{Table, Column, Foreign};
use types::Dao;
use meta::ModelMetaData;

use database::*;
use postgres::Connection;
use postgres::SslMode;
use regex::Regex;


pub struct Postgres {
	conn:Connection,
}


impl Postgres{
	
	//connection url
	pub fn new(url:&str)->Result<Self, &str>{
		let conn = Connection::connect(url, &SslMode::None);
		let res = match conn{
			Ok(c) => c,
			Err(e) => {return Err("Unable to connect to postgres database")} 
		};
		Ok(Postgres{conn:res})
	}
	
	
	/// get the rust data type names from database data type names
	/// will be used in source code generation
	fn dbtype_to_rust_type(db_type: &str)->(Option<Vec<String>>, String){
		let db_type = match db_type{
			"boolean" => {
				(None, "bool".to_string() )
			},
			"char" => {
				(None, "i8".to_string() )
			},
			"smallint" | "smallserial" => {
				(None, "i16".to_string() )
			},
			"integer" | "int" | "serial"  => {
				(None, "i32".to_string() )
			},
			"oid"  => {
				(None, "u32".to_string() )
			},
			"bigint" | "bigserial"  => {
				(None, "i64".to_string() )
			},
			"real" => {
				(None, "f32".to_string() )
			},
			"double precision" | "numeric" => {
				(None, "f64".to_string() )
			},
			"name" | "character" | "character varying" | "text" | "citext" =>{ 
				( None, "String".to_string() )
			},
			"bytea" =>{ 
				( None, "Vec<u8>".to_string() )
			},
			"json" | "jsonb" => {
				((Some(vec!["rustc_serialize::json::Json".to_string()]), "Json".to_string()))
			},
			"uuid" => {
				(Some(vec!["uuid::Uuid".to_string()]), "Uuid".to_string() )
			},
			"timestamp" => {
				(Some(vec!["chrono::naive::datetime::NaiveDateTime".to_string()]), "NaiveDateTime".to_string() )
			},
			"timestamp without time zone" => {
				(Some(vec!["chrono::naive::datetime::NaiveDateTime".to_string()]), "NaiveDateTime".to_string() )
			},
			"timestamp with time zone" => {
				(Some(vec!["chrono::datetime::DateTime".to_string(), 
						   "chrono::offset::utc::UTC".to_string()]), "DateTime<UTC>".to_string() )
			},
			"time with time zone" => {
				(Some(vec!["chrono::datetime::DateTime".to_string(), 
						   "chrono::offset::utc::UTC".to_string()]), "DateTime<UTC>".to_string() )
			},
			"date" => {
				(Some(vec!["chrono::naive::date::NaiveDate".to_string()]), "NaiveDate".to_string() )
			},
			"time" => {
				(Some(vec!["chrono::naive::time::NaiveTime".to_string()]), "NaiveTime".to_string() )
			},
			"hstore" => {
				((Some(vec!["std::collections::HashMap".to_string()]), "HashMap<String, Option<String>>".to_string()))
			},
			_ => panic!("Unable to get the equivalent data type for {}", db_type),
		};
		db_type
	}
	
	///
	/// convert rust data type names to database data type names
	/// will be used in generating SQL for table creation
	///
	fn rust_type_to_dbtype(rust_type: &str)->String{
		
		let rust_type = match rust_type{
			"bool" => {
				"boolean".to_string()
			},
			"i8" => {
				"char".to_string()
			},
			"i16" => {
				"smallint".to_string()
			},
			"i32"  => {
				"integer".to_string()
			},
			"u32"  => {
				"oid".to_string()
			},
			"i64"  => {
				"bigint".to_string()
			},
			"f32" => {
				"real".to_string()
			},
			"f64" => {
				"numeric".to_string()
			},
			"String" =>{ 
				"character varying".to_string()
			},
			"Vec<u8>" =>{ 
				"bytea".to_string()
			},
			"Json" => {
				"json".to_string()
			},
			"Uuid" => {
				"uuid".to_string()
			},
			"NaiveDateTime" => {
				"timestamp".to_string()
			},
			"DateTime<UTC>" => {
				"timestamp with time zone".to_string()
			},
			"NaiveDate" => {
				"date".to_string()
			},
			"NaiveTime" => {
				"time".to_string()
			},
			"HashMap<String, Option<String>>" => {
				"hstore".to_string()
			},
			_ => panic!("Unable to get the equivalent database data type for {}", rust_type),
		};
		rust_type
		
	}

	///
	/// http://stackoverflow.com/questions/109325/postgresql-describe-table
	///
	fn get_table_columns(&self, schema:&str, table:&str)->Vec<Column>{
		let sql = "
			SELECT  
			    pg_attribute.attnum AS number,  
			    pg_attribute.attname AS name,  
			    pg_attribute.attnotnull AS notnull,  
			    pg_catalog.format_type(pg_attribute.atttypid, pg_attribute.atttypmod) AS data_type,  
			    CASE  
				WHEN pg_constraint.contype = 'p' THEN true  
				ELSE false  
			    END AS is_primary,  
			    CASE  
				WHEN pg_constraint.contype = 'u' THEN true  
				ELSE false
			    END AS is_unique,
			    CASE
				WHEN pg_constraint.contype = 'f' THEN g.relname
			    END AS foreign_table,
			    CASE
				WHEN pg_attribute.atthasdef = true THEN pg_attrdef.adsrc
			    END as default
			    ,pg_description.description as comment
			    ,(SELECT nspname FROM pg_namespace WHERE oid=g.relnamespace) AS foreign_schema
			    ,(SELECT pg_attribute.attname FROM pg_attribute 
				WHERE pg_attribute.attrelid = pg_constraint.confrelid 
				AND pg_attribute.attnum = pg_constraint.confkey[1] 
				AND pg_attribute.attisdropped = false) AS foreign_column
			    ,pg_constraint.conname
			    
			FROM pg_attribute 
			    JOIN pg_class 
			        ON pg_class.oid = pg_attribute.attrelid  
			    JOIN pg_type 
			        ON pg_type.oid = pg_attribute.atttypid  
			    LEFT JOIN pg_attrdef 
					ON pg_attrdef.adrelid = pg_class.oid 
					AND pg_attrdef.adnum = pg_attribute.attnum  
			    LEFT JOIN pg_namespace 
					ON pg_namespace.oid = pg_class.relnamespace  
			    LEFT JOIN pg_constraint 
					ON pg_constraint.conrelid = pg_class.oid 
					AND pg_attribute.attnum = ANY (pg_constraint.conkey)  
			    LEFT JOIN pg_class AS g 
					ON pg_constraint.confrelid = g.oid  
			    LEFT JOIN pg_description 
					ON pg_description.objoid = pg_class.oid 
					AND pg_description.objsubid = pg_attribute.attnum 
			WHERE pg_class.relkind = 'r'::char  
			    AND pg_namespace.nspname = $1  
			    AND pg_class.relname = $2 
			    AND pg_attribute.attnum > 0
			    ORDER BY number
			";
			
		let stmt = self.conn.prepare(&sql).unwrap();
		let mut columns = Vec::new();
    	for row in stmt.query(&[&schema, &table]).unwrap() {
	    	let name:String = row.get("name");
	    	let not_null:bool = row.get("notnull");
	    	let data_type:String = row.get("data_type");
	    	//TODO: temporarily regex the data type to extract the size as well
	    	let re = match Regex::new("(.+)\\((.+)\\)") {
		 		Ok(re) => re,
	     		Err(err) => panic!("{}", err),
			};
	    	
	    	let data_type = if re.is_match(&data_type){
	    		let cap = re.captures(&data_type).unwrap();
	    		let data_type = cap.at(1).unwrap().to_string();
	    		let size = cap.at(2).unwrap().to_string();//TODO::can be use in the later future
	    		data_type
	    	}else{
    			data_type
    		};
	    	
	    	let is_primary:bool = row.get("is_primary");
	    	let is_unique:bool = row.get("is_unique");
	    	
	    	let default:Option<String> = match row.get_opt("default"){
	    			Ok(x) => Some(x),
	    			Err(_) => None
	    		};
	    	let comment:Option<String> = match row.get_opt("comment"){
    				Ok(x) => Some(x),
    				Err(_) => None
    			};
	    	
	    	let foreign_schema:Option<String> = match row.get_opt("foreign_schema"){
    				Ok(x) => Some(x),
    				Err(_) => None
    			};
	    	let foreign_column:Option<String> = match row.get_opt("foreign_column"){
	    			Ok(x) => Some(x),
	    			Err(_) => None
	    		};
	    	let foreign_table:Option<String> = match row.get_opt("foreign_table"){
	    			Ok(x) => Some(x),
	    			Err(_) => None
    			};
	    	
	    	
	    	let foreign = if foreign_table.is_some() && 
				    		 foreign_column.is_some() && 
					    	 foreign_schema.is_some(){
					    		Some(
					    			Foreign{
					    				schema:foreign_schema.unwrap(),
					    				table:foreign_table.unwrap(),
					    				column:foreign_column.unwrap()
				    			})
    				
				    		}else{
								None
							};
	    	
	    	let column = Column{
	    			name:name,
	    			data_type:data_type,
	    			comment:comment,
	    			is_primary:is_primary,
	    			is_unique:is_unique,
	    			default:default,
	    			not_null:not_null,
	    			foreign:foreign,
	    			is_inherited:false,//will be corrected later in the get_meta_data
	    		};
	    	columns.push(column);
    	}
    	//unify due to the fact that postgresql return a separate row for 
    	// both primary and foreign columns
		Self::unify_primary_and_foreign_column(&columns)
	}
	
	/// column that is both primary and foreign should be unified
	fn unify_primary_and_foreign_column(columns:&Vec<Column>)->Vec<Column>{
		let mut unified_columns = Vec::new();
		let mut primary_columns = Vec::new();
		let mut foreign_columns = Vec::new();
		for c in columns{
			if c.is_primary{
				primary_columns.push(c.name.clone());
			}
			if c.foreign.is_some(){
				foreign_columns.push(c.name.clone());
			}
		}
		//if both primary and foreign, push only the modified foreign
		for c in columns{
			if primary_columns.contains(&c.name) && foreign_columns.contains(&c.name){
				if c.foreign.is_some(){
					let mut clone_column = c.clone();
					clone_column.is_primary = true;
					unified_columns.push(clone_column);
				}
			}
			else{
				unified_columns.push(c.clone());
			}	
		}
		unified_columns

	}
	
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
	fn retrieve(&self, query:&Query)->Vec<Dao>{Vec::new()}
	fn update(&self, dao:Dao, model:&Table, filters:&Vec<Filter>)->Dao{panic!("not yet")}
	fn delete(&self, model:&Table, filters:&Vec<Filter>)->usize{0}
	fn empty(&self, model:&Table, forced:bool)->usize{0}
	fn write_to_blob(&self, buffer:Vec<u8>)->usize{0}
	fn write_to_file(&self, filename:&String){}
	fn get_blob(&self, oid:u64)->Vec<u8>{Vec::new()}
	fn insert(&self, dao:&Dao, meta:&ModelMetaData, model:&Table, query:&Query)->Dao{panic!("not yet")}
	fn search(&self, query:&Query, keyword:String){}
	fn correct_data_types(&self, dao_list:Vec<Dao>, model:&Table){}
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

/// this can be condensed with using just extracting the table definition
impl DatabaseDev for Postgres{
	
	fn get_parent_table(&self, schema:&str, table:&str)->Option<String>{
		let sql ="
			SELECT 
				relname as table,  
				pg_namespace.nspname as schema,
				( SELECT relname FROM pg_class WHERE oid = pg_inherits.inhparent ) AS parent_table 
			 FROM pg_class  
			 INNER JOIN pg_namespace
				ON pg_class.relnamespace = pg_namespace.oid
			 LEFT JOIN pg_inherits  
			 	ON pg_class.oid = pg_inherits.inhrelid 
			 WHERE pg_namespace.nspname = $1
				 AND relname = $2
				";
				
		let stmt = self.conn.prepare(&sql).unwrap();
    	for row in stmt.query(&[&schema, &table]).unwrap() {	
    		let parent_table:Option<String> = match row.get_opt("parent_table"){
    				Ok(x) => Some(x),
    				Err(_) => None
    			};
    		return parent_table;
    	}
    	None
	}
	
	fn get_table_sub_class(&self, schema:&str, table:&str)->Option<Vec<String>>{
		let sql ="
			SELECT 
				relname AS base_table, 
				( SELECT relname FROM pg_class WHERE oid = pg_inherits.inhrelid ) AS sub_class 
			 FROM pg_inherits 
			 LEFT JOIN pg_class 
				ON pg_class.oid = pg_inherits.inhparent 
			 INNER JOIN pg_namespace
				ON pg_class.relnamespace = pg_namespace.oid
			 WHERE pg_namespace.nspname = $1
			 AND relname = $2
			 ORDER BY relname
			";
				
		let stmt = self.conn.prepare(&sql).unwrap();
		let mut sub_classes:Vec<String> = Vec::new();
    	for row in stmt.query(&[&schema, &table]).unwrap() {	
    		match row.get_opt("sub_class"){
					Ok(x) => sub_classes.push(x),
					Err(_) => (),
				};
    	}
    	if sub_classes.len() > 0 {
			return Some(sub_classes)
		}
    	None
	}
	

	
	fn get_table_metadata(&self, schema:&str, table:&str)->Table{
		
		let mut columns = self.get_table_columns(schema, table);
		let comment = self.get_table_comment(schema, table);
		let parent = self.get_parent_table(schema, table);
		let subclass = self.get_table_sub_class(schema, table);
		
		//mutate columns to mark those which are inherited
		if parent.is_some(){
			let inherited_columns = self.get_inherited_columns(schema, table);
			for i in inherited_columns{
				for c in &mut columns{
					if i == c.name{
						c.is_inherited = true;
					}
				}
			}
		}
		
		Table{
			schema:schema.to_string(),
			name:table.to_string(),
			parent_table:parent,
			sub_table:subclass,
			comment:comment,
			columns:columns,
		}
	}
	
	fn get_all_tables(&self)->Vec<(String, String)>{
		let sql ="
				SELECT 
					pg_class.relname AS table, 
					pg_namespace.nspname AS schema, 
					obj_description(pg_class.oid) AS comment 
				FROM pg_class
					LEFT JOIN pg_namespace 
						ON pg_namespace.oid = pg_class.relnamespace
				WHERE 
					pg_class.relkind = 'r'
					AND pg_namespace.nspname NOT IN ('information_schema', 'pg_catalog', 'pg_toast')
				ORDER BY relname, nspname
					
				";
		let stmt = self.conn.prepare(&sql).unwrap();
		let mut tables:Vec<(String, String)> = Vec::new();
    	for row in stmt.query(&[]).unwrap() {	
    		let table:String = row.get("table");
    		let schema:String = row.get("schema");
    		tables.push((schema, table));
    	}
    	tables
	}
	
	fn get_table_comment(&self, schema:&str, table:&str)->Option<String>{
		let sql ="
				SELECT 
					pg_class.relname AS table, 
					pg_namespace.nspname AS schema, 
					obj_description(pg_class.oid) AS comment 
				FROM pg_class
					LEFT JOIN pg_namespace 
						ON pg_namespace.oid = pg_class.relnamespace
				WHERE 
					pg_class.relkind = 'r'
					AND pg_namespace.nspname NOT IN ('information_schema', 'pg_catalog', 'pg_toast')
					AND nspname = $1
					AND relname = $2
				";
				
		let stmt = self.conn.prepare(&sql).unwrap();
    	for row in stmt.query(&[&schema, &table]).unwrap() {	
    		let comment:Option<String> = match row.get_opt("comment"){
					Ok(x) => Some(x),
					Err(_) => None
				};
    		return comment;
    	}
    	None
	}
	
	fn get_inherited_columns(&self, schema:&str, table:&str)->Vec<String>{
		let sql = "
				SELECT nmsp_parent.nspname    AS parent_schema,
					parent.relname         AS parent_table,
					nmsp_child.nspname     AS child_schema,
				   	child.relname          AS child_table,           
				   	column_parent.attname  AS column_parent_name
				FROM pg_inherits
					JOIN pg_class parent
						ON pg_inherits.inhparent  = parent.oid
					JOIN pg_class child             
						ON pg_inherits.inhrelid   = child.oid
					JOIN pg_namespace nmsp_parent   
						ON nmsp_parent.oid        = parent.relnamespace
					JOIN pg_namespace nmsp_child    
						ON nmsp_child.oid         = child.relnamespace
					JOIN pg_attribute column_parent 
						ON column_parent.attrelid = parent.oid
					WHERE column_parent.attnum > 0
					AND nmsp_child.nspname = $1
					AND child.relname = $2
					ORDER BY column_parent.attname
				";
		let stmt = self.conn.prepare(&sql).unwrap();
		let mut inherited_columns = Vec::new();
    	for row in stmt.query(&[&schema, &table]).unwrap() {	
    		let column:String = row.get("column_parent_name");
    		inherited_columns.push(column);
    	}
    	inherited_columns
	}
	
	
	fn get_rust_data_type(&self, db_type:&str)->(Option<Vec<String>>, String){
		Postgres::dbtype_to_rust_type(db_type)
	}
	
	
}

