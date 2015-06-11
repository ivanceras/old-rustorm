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

    /// execute a plain sql string
    fn execute_sql(&self, sql:&String, param:&Vec<String>)->usize;
    
    /// execute a query object
    fn execute_query(&self, query:Query)->usize;

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
    fn get_table_sub_class(&self, schema:&str, table:&str)->Vec<String>;
    
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
    fn get_rust_data_type(&self, db_type:&str)->(Vec<String>, String);


    /// build a source code for the struct defined by this table
    ///(imports, imported_tables, source code)
    fn to_struct_source_code<'a>(&self, table:&'a Table, all_tables:&'a Vec<Table>)->(Vec<String>, Vec<&'a Table>, String){
        let mut w = Writer::new();
        //imported tables needed since we are partitioning the tables in schemas
        let mut imported_tables = Vec::new();
        //imports
        let mut imports:Vec<String> = Vec::new();
        for c in &table.columns{
            let (package, _) = self.get_rust_data_type(&c.db_data_type);
            if !package.is_empty(){
                for i in package{
                    imports.push(i);
                }
            }
        }
        imports.sort_by(|a, b| a.cmp(b));
        imports.dedup();
        
        //struct
        let struct_name = table.struct_name();
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
        w.append("#[derive(RustcDecodable, RustcEncodable)]");
        w.ln();
        w.append("#[derive(Debug)]");
        w.ln();
        w.append("pub struct ").append(&struct_name).appendln(" {");
        
        let mut included_columns = Vec::new();
        //primary columns
        for p in table.primary_columns(){
            if !included_columns.contains(&p.name){
                w.tab();
                w.append("/// primary");
                w.ln();
                Self::write_column(&mut w, p);
                included_columns.push(p.name.clone());
            }
        }
        //unique columns
        for u in table.unique_columns(){
            if !included_columns.contains(&u.name){
                w.tab();
                w.append("/// unique");
                w.ln();
                Self::write_column(&mut w, u);
                included_columns.push(u.name.clone());
            }
        }
        
        // uninherited columns
        for uc in &table.uninherited_columns(){
            if !included_columns.contains(&uc.name){
                Self::write_column(&mut w, uc);
                included_columns.push(uc.name.clone());
            }
        }
        
        // inherited columns
        for ic in &table.inherited_columns(){
            if !included_columns.contains(&ic.name){
                Self::write_column(&mut w, ic);
                included_columns.push(ic.name.clone());
            }
        }
        
        // foreign columns, has_one
        let foreign_tables = table.referred_tables(all_tables);
        for (column, ft) in foreign_tables{
            let condense_name = column.condense_name();
            let member_name = if included_columns.contains(&condense_name){
                format!("{}_HasOne",column.name)
            }
            else{
                condense_name
            };
            
            if ft.name != table.name { //do not include self referencial table
                w.tab();
                w.append("/// has one");
                w.ln();
                w.tab();
                w.append("pub ");
                w.append(&member_name);
                included_columns.push(member_name.clone());
                w.append(":");
                w.append("Option<");
                w.append(&ft.struct_name());
                imported_tables.push(ft);
                w.append(">");
                w.comma();
                w.ln();
            }else{
                w.tab();
                w.append("/// has one, self referential");
                w.ln();
                w.tab();
                w.append("pub ");
                w.append(&member_name);
                included_columns.push(member_name.clone());
                w.append(":");
                w.append("Option<");
                w.append("Box<");
                w.append(&ft.struct_name());
                w.append(">");
                w.append(">");
                w.comma();
                w.ln();
            }
        }
        
        //extension tables
        let extension_tables = table.extension_tables(all_tables);
        let mut included_ext = Vec::new();
        for ext in &extension_tables{
            if !included_ext.contains(&&ext.name){
                w.tab();
                w.append("/// has one, extension table");
                w.ln();
                w.tab();
                w.append("pub ");
                let member_name = if included_columns.contains(&ext.name){
                    format!("{}_Ext",&ext.name)
                }else{
                    ext.name.to_string()
                };
                w.append(&member_name);
                w.append(":");
                w.append("Option<");
                w.append("Box<");//put it inside the box to get rid of illegal recursive struct
                w.append(&ext.struct_name());
                imported_tables.push(ext);
                w.append(">");
                w.append(">");
                w.comma();
                w.ln();
                included_ext.push(&ext.name);//put to included in hasMany to prevent it from putting it there
            }
        }
        //indirect referring table
        let mut linker_tables = Vec::new();
        for (indirect, linker_table) in table.indirect_referring_tables(all_tables){
                linker_tables.push(linker_table);
                w.tab();
                w.append("/// has many, indirect referring table, derived from linker table: ");
                w.append(&linker_table.name);
                w.ln();
                w.tab();
                w.append("pub ");
                 let member_name = if included_columns.contains(&indirect.name){
                    format!("{}_Indirect",&indirect.name)
                }else{
                    indirect.name.to_string()
                };
                w.append(&member_name);
                w.append(":");
                w.append("Vec<");//put it inside the box to get rid of illegal recursive struct
                w.append(&indirect.struct_name());
                imported_tables.push(indirect);
                w.append(">");
                w.comma();
                w.ln();
                //included_has_many.push(&indirect.name);//put to included in hasMany to prevent it from putting it there
        }
        
        // referring table, has_many
        let mut included_has_many = Vec::new();
        for (ref_table, _) in table.referring_tables(all_tables){
            if !linker_tables.contains(&ref_table) &&
               !extension_tables.contains(&ref_table) &&
               !included_has_many.contains(&&ref_table.name){
                w.tab();
                w.append("/// has many");
                w.ln();
                w.tab();
                w.append("pub ");
                //just appending has_many when there is a column name with that table already
                let member_name = if included_columns.contains(&ref_table.name){
                    format!("{}_HasMany",&ref_table.name)
                }else{
                    ref_table.name.to_string()
                };
                w.append(&member_name);
                included_columns.push(member_name.clone());
                w.append(":");
                w.append("Vec<");
                w.append(&ref_table.struct_name());
                imported_tables.push(ref_table);
                w.append(">");
                w.comma();
                w.ln();
                included_has_many.push(&ref_table.name);
            }
        }
        
        w.append("}");
        w.ln();
        imported_tables.sort_by(|a, b| (a.long_name().cmp(&b.long_name())));
        imported_tables.dedup();
        
        (imports, imported_tables, w.src)
    }
    
    fn write_column(w:&mut Writer, c:&Column){
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
                w.append("/// default: ");
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
        //let (_, data_type) = self.get_rust_data_type(&c.db_data_type);
        w.tab();
        w.append("/// db data type: ");
        w.append(&c.db_data_type);
        w.ln();

        w.tab();
        w.append("pub ");
        w.append(&c.corrected_name());
        w.append(":");
        if c.not_null{
            w.append(&c.data_type);
        }else{
            w.append("Option<");
            w.append(&c.data_type);
            w.append(">");
        }
        w.comma();
        w.ln();
    }
    
    
}