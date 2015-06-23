use query::Query;
use table::{Table, Column};
use dao::Dao;
use writer::Writer;
use dao::Type;
use filter::{Connector, Equality, Operand};

/// A lower level API for manipulating objects in the database
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

    /// select
    /// returns an array to the qualified records
    fn select(&self, query:&Query)->Vec<Dao>;

    /// insert
    /// insert an object, returns the inserted Dao value
    /// including the value generated via the defaults
    fn insert(&self, query:&Query)->Dao;

    /// update
    /// returns the updated Dao
    fn update(&self,query:&Query)->Dao;

    /// delete records
    /// returns the number of deleted records
    fn delete(&self, query:&Query)->Result<u64, &str>;

    /// DDL executions
    /// simple sql string with no parameters and no return
    /// create, alter, drop, truncate, rename, constraint
    /// returns error if error occurs
    fn execute_ddl(&self, sql:&String)->Result<(), &str>;

    /// everything else
    fn execute_sql(&self, sql:&String, param:&Vec<String>)->Result<u64, &str>;

    /// Actually converting the from whatever JDBC converts the object to the correct type that we intend to be using
    fn correct_data_types(&self, dao_list:Vec<Dao>, model:&Table);


    /// build a query, return the sql string and the parameters.
    fn build_query(&self, query:&Query)->(String, Vec<Type>);
    
    /// build the filter clause or the where clause of the query
    fn build_filters(&self, query: &Query)->(String, Vec<Type>){
        let mut params = vec![];
        let mut w = Writer::new();
        let mut do_connector = false;
        for filter in &query.filters{
            if do_connector{
                match filter.connector{
                    Connector::And => w.append("AND "),
                    Connector::Or => w.append("OR "),
                };
            }else{
                do_connector = true;
            }
            w.append(&filter.column);
            w.append(" ");
            match filter.equality{
                Equality::EQ => w.append("= "),
                Equality::NE => w.append("!= "),
                Equality::LT => w.append("< "),
                Equality::LTE => w.append("<= "),
                Equality::GT => w.append("> "),
                Equality::GTE => w.append(">= "),
                Equality::IN => w.append("IN "),
                Equality::NOTIN => w.append("NOT IN "),
                Equality::LIKE => w.append("LIKE "),
                Equality::NULL => w.append("IS NULL "),
                Equality::NOTNULL => w.append("IS NOT NULL "),
                Equality::ISNULL => w.append("IS NULL "),
            };
            
            match filter.operand{
                Operand::Query(ref q) => {
                    let (sql, param) = self.build_query(q);
                    for p in param{//TODO: change to params.append(param) in next releases of rust
                        params.push(p);
                    }
                    w.append(&sql);
                },
                Operand::Value(ref t) => {
                    w.append("$1"); //TODO: fix numbered parameters according to the order of param
                    params.push(t.clone());
                },
            };
        }
        (w.src, params)
    }

    /// TODO include filters, joins, groups, paging
    fn build_select(&self, query: &Query)->(String, Vec<Type>){
        println!("building select query");
        let mut params = vec![];
        let mut w = Writer::new();
        w.append("SELECT ");
        let mut do_comma = false;
        let mut cnt = 0;
        for ec in &query.enumerated_columns{
            if do_comma{w.comma();}else{do_comma=true;}
            cnt += 1;
            if cnt % 5 == 0{//break at every 5 columns to encourage sql tuning/revising
                w.ln_tab();
            }
            w.append(" ");
            w.append(&ec.column);
        }
        w.ln();
        w.append(" FROM ");
        assert!(query.from_table.is_some());
        let table_name = query.from_table.clone().unwrap().complete_name();
        w.append(&table_name);
        if query.filters.len() > 0 {
            w.append(" WHERE ");
            let (fsql, fparam) = self.build_filters(query);
            w.append(&fsql);
            for fp in fparam{
                params.push(fp);
            }
        }
        (w.src, params)
    }
    
    /// TODO complete this
    fn build_insert(&self, query: &Query)->(String, Vec<Type>){
        println!("building select query");
        let mut w = Writer::new();
        w.append("INSERT INTO");
        let mut do_comma = false;
        let mut cnt = 0;
        for ec in &query.enumerated_columns{
            if do_comma{w.comma();}else{do_comma=true;}
            cnt += 1;
            if cnt % 5 == 0{//break at every 5 columns to encourage sql tuning/revising
                w.ln_tab();
            }
            w.append(" ");
            w.append(&ec.column);
        }
        w.ln();
        w.append(" FROM ");
        assert!(query.from_table.is_some());
        let table_name = query.from_table.clone().unwrap().complete_name();
        w.append(&table_name);
        (w.src, vec![])
    }

    ///TODO :complete this
    fn build_update(&self, query: &Query)->(String, Vec<Type>){
        println!("building select query");
        let mut w = Writer::new();
        w.append("UPDATE ");
        let mut do_comma = false;
        let mut cnt = 0;
        for ec in &query.enumerated_columns{
            if do_comma{w.comma();}else{do_comma=true;}
            cnt += 1;
            if cnt % 5 == 0{//break at every 5 columns to encourage sql tuning/revising
                w.ln_tab();
            }
            w.append(" ");
            w.append(&ec.column);
        }
        w.ln();
        w.append(" FROM ");
        assert!(query.from_table.is_some());
        let table_name = query.from_table.clone().unwrap().complete_name();
        w.append(&table_name);
        (w.src, vec![])
    }

    fn build_delete(&self, query: &Query)->(String, Vec<Type>){
        println!("building select query");
        let mut w = Writer::new();
        w.append("DELETE ");
        w.append(" FROM ");
        assert!(query.from_table.is_some());
        w.append("WHERE");
        let table_name = query.from_table.clone().unwrap().complete_name();
        w.append(&table_name);
        (w.src, vec![])
    }

    #[test]
    fn test_build_select(){
    
    }

}

pub trait DatabaseDDL{
    //////////////////////////////////////////
    /// The following methods involves DDL(Data definition language) operation
    ////////////////////////////////////////

    /// create a database schema
    fn create_schema(&self, schema:&str);

    /// drop the database schema
    fn drop_schema(&self, schema:&str, forced:bool);

    /// create a database table based on the Model Definition
    fn create_table(&self, model:&Table);

    /// rename table, in the same schema
    fn rename_table(&self, table:&Table, new_tablename:String);

    /// drop table
    fn drop_table(&self, table:&Table, forced:bool);

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
        
        let referenced_table = table.get_all_referenced_table(all_tables);
        for ref_table in referenced_table{
            w.ln_tab();
            w.append("/// ");
            let comment = if ref_table.is_has_one{
                if ref_table.table != table{
                    "has one"
                }
                else{
                    "has one, self referential"
                }
            }else if ref_table.is_ext{
                "has one, extension table"
            }
            else if ref_table.is_has_many{
                if ref_table.is_direct{
                    "has many"
                }else{
                    "has mny, indirect"
                }
            }
            else{
                "unreachable!"
            };
            w.append(comment);
            w.ln_tab();
            let member_name = ref_table.member_name(table);
            w.append("pub ");
            w.append(&member_name);
            w.append(": ");
            if ref_table.is_has_one {
                if ref_table.table != table{
                    w.append("Option<");
                    w.append(&ref_table.table.struct_name());
                    w.append(">");
                }else{
                    w.append("Option<Box<");
                    w.append(&ref_table.table.struct_name());
                    w.append(">>");
                }
            }
            if ref_table.is_ext{
                w.append("Option<Box<");
                w.append(&ref_table.table.struct_name());
                w.append(">>");
            }
            if ref_table.is_has_many{
                w.append("Vec<");//put it inside the box to get rid of illegal recursive struct
                w.append(&ref_table.table.struct_name());
                w.append(">");
            }
            w.comma();
            imported_tables.push(ref_table.table);
            
        }
        w.ln();
        w.append("}");
        w.ln();
        imported_tables.sort_by(|a, b| (a.complete_name().cmp(&b.complete_name())));
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
