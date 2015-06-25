use query::Query;
use table::{Table, Column};
use dao::Dao;
use writer::Writer;
use dao::{Type, DaoResult};
use query::{Connector, Equality, Operand};
use query::{Direction, Modifier, JoinType};
use query::Filter;

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
    fn select(&self, query:&Query)->DaoResult;

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
    
    /// build operand, i.e: columns, query, function, values
    fn build_operand(&self, operand:&Operand, param_count:usize)->(String, Vec<Type>){
        let mut w = Writer::new();
        let mut params = vec![];
        let mut param_count = param_count;
        match operand{
                &Operand::Column(ref column) => {
                    w.append(&column.column); //TODO: needs to do complete/super complete name when there is possible conflict of column names
                }, 
                &Operand::Function(ref function)=>{
                        w.append("(");
                        let mut do_comma = false;
                        for param in &function.params{
                            if do_comma{ w.commasp(); }else{ do_comma = true;}
                            let (operand_sql, operand_params) = self.build_operand(param, param_count);
                            w.append(&operand_sql);
                            param_count += operand_params.len();
                            for op in operand_params{
                                params.push(op);
                            }
                        }
                        w.append(")");
                    },
                &Operand::Query(ref q) => {
                    let (sql, param) = self.build_query(q);
                    for p in param{//TODO: change to params.append(param) in next releases of rust
                        params.push(p);
                    }
                    w.append(&sql);
                },
                &Operand::Value(ref value) => {
                    param_count += 1;
                    let numbered_param = format!("${} ",param_count);
                    w.append(&numbered_param); //TODO: fix numbered parameters according to the order of param
                    params.push(value.clone());
                },
            };
        (w.src, params)
    }
    
    
    fn build_filter(&self, filter:&Filter, param_count:usize)->(String, Vec<Type>){
        let mut param_count = param_count;
        let mut params = vec![];
        let mut w = Writer::new();
        let (left_sql, left_params) = self.build_operand(&filter.left_operand, param_count);
        param_count += left_params.len();
        w.append(&left_sql);
        for lp in left_params{
            params.push(lp);
        }
        
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
        
        let (right_sql, right_params) = self.build_operand(&filter.right_operand, param_count);
        param_count += right_params.len();
        w.append(&right_sql);
        for lp in right_params{
            params.push(lp);
        }
        (w.src, params)
    }
    
    /// build the filter clause or the where clause of the query
    /// TODO: add the sub filters
    fn build_filters(&self, filters: &Vec<Filter>, param_count: usize)->(String, Vec<Type>){
        let mut param_count = param_count;
        let mut params = vec![];
        let mut w = Writer::new();
        let mut do_connector = false;
        for filter in filters{
            if do_connector{
                w.ln_tabs(2);
                match filter.connector{
                    Connector::And => w.append("AND "),
                    Connector::Or => w.append("OR "),
                };
            }else{
                do_connector = true;
            }
            let (filter_sql, filter_params) = self.build_filter(filter, param_count);
            param_count += filter_params.len();
            w.append(&filter_sql);
            for fp in filter_params{
                params.push(fp);
            }
        }
        (w.src, params)
    }

    /// build the enumerated, distinct, *, columns
    fn build_columns(&self, query: &Query)->(String, Vec<Type>){
        let mut w = Writer::new();
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
        (w.src, vec![])
    }

    /// TODO include filters, joins, groups, paging
    fn build_select(&self, query: &Query)->(String, Vec<Type>){
        println!("building select query");
        let mut params = vec![];
        let mut w = Writer::new();
        w.append("SELECT ");
        let (column_sql, _) = self.build_columns(query); //TODO: add support for column_sql, fields, functions
        w.append(&column_sql);
        w.ln();
        w.append(" FROM ");
        assert!(query.from_table.is_some());
        let table_name = query.from_table.clone().unwrap().complete_name();
        w.append(&table_name);
        w.append(" ");
        if !query.joins.is_empty(){
            w.ln_tab();
            for join in &query.joins{
                match join.modifier{
                    Some(ref modifier) => {
                            match modifier{
                                &Modifier::LEFT => w.append("LEFT "),
                                &Modifier::RIGHT => w.append("RIGHT "),
                                &Modifier::FULL => w.append("FULL "),
                            };
                        },
                    None => ()
                };
                
                match join.join_type{
                    JoinType::CROSS => w.append("CROSS "),
                    JoinType::INNER => w.append("INNER "),
                    JoinType::OUTER => w.append("OUTER "),
                };
                w.append("JOIN ");
                w.append(&join.table_name.complete_name());
                w.append(" ");
                assert!(join.column1.len() == join.column2.len(), "There should be equal number of corresponding columns to join");
                let mut cnt = 0;
                let mut do_and = false;
                for jc in &join.column1{
                    w.ln_tabs(2);
                    if do_and {
                        w.append("AND ");
                    }else{
                        w.append("ON ");
                        do_and = true;
                    }
                    w.append(jc);
                    w.append(" = ");
                    w.append(&join.column2[cnt]);
                    w.append(" ");
                    cnt += 1;
                }
            }
        }
        
        if !query.filters.is_empty() {
            w.ln_tab();
            w.append("WHERE ");
            let (fsql, fparam) = self.build_filters(&query.filters, 0);
            w.append(&fsql);
            for fp in fparam{
                params.push(fp);
            }
        }
        
        if !query.grouped_columns.is_empty() {
            w.ln_tab();
            w.append("GROUP BY ");
            let mut do_comma = false;
            for column in &query.grouped_columns{
                if do_comma{ w.comma(); }else{ do_comma = true;}
                w.append(column);
                w.append(" ");
            }
        };
        
        if !query.order_by.is_empty(){
            w.ln_tab();
            w.append("ORDER BY ");
            let mut do_comma = false;
            for &(ref column, ref direction) in &query.order_by{
                if do_comma { w.commasp();} else { do_comma = true;}
                w.append(&column);
                match direction{
                    &Direction::ASC => w.append(" ASC"),
                    &Direction::DESC => w.append(" DESC")
                };
            }
        };
        
        match query.page_size{
            Some(page_size) => {
                w.ln_tab();
                w.append("LIMIT ");
                w.append(&format!("{}",page_size));
            },
            None => (),
        };
        
        match query.page{
            Some(page) =>{
                w.ln_tab();
                w.append("OFFSET ");
                assert!(query.page_size.is_some(), "Page size should be specified when paging");
                let page_size = query.page_size.unwrap();
                let offset = page * page_size;
                w.append(&format!("{}",offset));
            },
            None => (),
        };
        
        (w.src, params)
    }
    
    /// TODO complete this
    fn build_insert(&self, query: &Query)->(String, Vec<Type>){
        println!("building select query");
        let mut w = Writer::new();
        w.append("INSERT INTO");
        let (column_sql, _) = self.build_columns(query); //TODO: add support for column_sql, fields, functions
        w.append(&column_sql);
        w.ln();
        w.append(" FROM ");
        assert!(query.from_table.is_some());
        let table_name = query.from_table.clone().unwrap().complete_name();
        w.append(&table_name);
        (w.src, vec![])
    }

    ///TODO :complete this
    fn build_update(&self, query: &Query)->(String, Vec<Type>){
        println!("building update query");
        let mut params = vec![];
        let mut w = Writer::new();
        w.append("UPDATE ");
        let (column_sql, _) = self.build_columns(query); //TODO: add support for column_sql, fields, functions
        w.append(&column_sql);
        w.ln();
       if !query.filters.is_empty() {
            w.ln_tab();
            w.append("WHERE ");
            let (fsql, fparam) = self.build_filters(&query.filters, 0);
            w.append(&fsql);
            for fp in fparam{
                params.push(fp);
            }
        }
        (w.src, params)
    }

    fn build_delete(&self, query: &Query)->(String, Vec<Type>){
        println!("building delete query");
        let mut params = vec![];
        let mut w = Writer::new();
        w.append("DELETE FROM");
        assert!(query.from_table.is_some());
        if !query.filters.is_empty() {
            w.ln_tab();
            w.append("WHERE ");
            let (fsql, fparam) = self.build_filters(&query.filters, 0);
            w.append(&fsql);
            for fp in fparam{
                params.push(fp);
            }
        }
        (w.src, params)
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
                    "has many, indirect"
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
