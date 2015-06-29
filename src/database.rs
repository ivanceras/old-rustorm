use query::Query;
use table::{Table, Column};
use dao::{Dao,DaoResult, Type};
use writer::{SqlFrag, Writer};
use query::{Connector, Equality, Operand, Field};
use query::{Direction, Modifier, JoinType};
use query::Filter;


/// SqlOption, contains the info about the features and quirks of underlying database
#[derive(PartialEq)]
pub enum SqlOption{
    /// use the numbered parameters, as the case with rust-postgres
    UseNumberedParam,
    /// sqlite, jdbc
    UseQuestionMark,
    /// postgresql supports returning clause on insert and update
    SupportsReturningClause,
    /// support CTE (common table expression ie. WITH)
    SupportsCTE,
}

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
    fn delete(&self, query:&Query)->Result<usize, String>;

    /// execute query with return dao
    fn execute_with_return(&self, query:&Query)->DaoResult;
    
    /// execute query with no return dao
    fn execute(&self, query:&Query)->Result<usize, String>;

    /// execute insert with returning clause, update with returning clause
    fn execute_sql_with_return(&self, sql:&String, params:&Vec<Type>)->Vec<Dao>;
    
    /// everything else
    fn execute_sql(&self, sql:&String, param:&Vec<Type>)->Result<usize, String>;

    /// build a query, return the sql string and the parameters.
    fn build_query(&self, query:&Query)->SqlFrag;
    
    /// build operand, i.e: columns, query, function, values
    fn build_operand(&self, w: &mut SqlFrag, parent_query:&Query, operand:&Operand){
        match operand{
            &Operand::ColumnName(ref column_name) => {
                if parent_query.joins.is_empty(){
                    w.append(&column_name.column);
                }else{
                    w.append(&column_name.complete_name());
                }
            }, 
            &Operand::TableName(ref table_name) => {
                w.append(&table_name.complete_name());
            },
            &Operand::Function(ref function)=>{
                    w.append("(");
                    let mut do_comma = false;
                    for param in &function.params{
                        if do_comma{ w.commasp(); }else{ do_comma = true;}
                        self.build_operand(w, parent_query, param);
                    }
                    w.append(")");
                },
            &Operand::Query(ref q) => {
                let sql_frag = self.build_query(q);
                w.append(&sql_frag.sql);
            },
            &Operand::Value(ref value) => {
                w.parameter(value.clone());
            },
            &Operand::Vec(ref operands) => {
                let mut do_comma = false;
                if !operands.is_empty(){
                    w.append("(");
                    for op in operands{
                        println!("op: {:?}",op);
                        if do_comma {w.commasp();}else{do_comma = true;}
                        self.build_operand(w, parent_query, op);
                    }
                    w.append(")");
                }
            },
        };
    }
    
    fn build_field(&self, w: &mut SqlFrag, parent_query:&Query, field:&Field){
        self.build_operand(w, parent_query, &field.operand);
        match field.name{
            Some(ref name) => {
                w.append("AS ");
                w.append(name);
            }
            None => (),
        };
    }
    
    
    fn build_filter(&self, w: &mut SqlFrag, parent_query:&Query, filter:&Filter){
        self.build_operand(w, parent_query, &filter.left_operand);
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
        self.build_operand(w, parent_query, &filter.right_operand);
    }
    
    /// build the filter clause or the where clause of the query
    /// TODO: add the sub filters
    fn build_filters(&self, w: &mut SqlFrag, parent_query:&Query, filters: &Vec<Filter>){
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
            self.build_filter(w, parent_query, filter);
        }
    }

    /// build the enumerated, distinct, *, columns
    fn build_enumerated_fields(&self, w: &mut SqlFrag, parent_query:&Query, enumerated_fields: &Vec<Field>){
        let mut do_comma = false;
        let mut cnt = 0;
        for field in enumerated_fields{
            if do_comma{w.commasp();}else{do_comma=true;}
            cnt += 1;
            if cnt % 5 == 0{//break at every 5 columns to encourage sql tuning/revising
                w.ln_tab();
            }
            self.build_field(w, parent_query, field);
        }
    }

    /// TODO include filters, joins, groups, paging
    fn build_select(&self, query: &Query)->SqlFrag{
        println!("building select query");
        let mut w = SqlFrag::new(self.sql_options());
        w.append("SELECT ");
        self.build_enumerated_fields(&mut w, query, &query.enumerated_fields); //TODO: add support for column_sql, fields, functions
        w.ln();
        w.append(" FROM ");
        
        assert!(query.from.is_some(), "There should be table, query, function to select from");
        
        match query.from{
            Some(ref field) => {
                self.build_field(&mut w, query, field);
            }
            None => println!("Warning: No from in this query"),
        };
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
            self.build_filters(&mut w, query, &query.filters);
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
        w
    }
    
    /// TODO complete this
    fn build_insert(&self, query: &Query)->SqlFrag{
        println!("building insert query");
        let mut w = SqlFrag::new(self.sql_options());
        w.append("INSERT INTO ");
        let into_table = query.get_from_table();
        assert!(into_table.is_some(), "There should be table to insert to");
        if into_table.is_some(){
            w.append(&into_table.unwrap().complete_name());
        }
        
        w.append("(");
        self.build_enumerated_fields(&mut w, query, &query.enumerated_fields); //TODO: add support for column_sql, fields, functions
        w.append(") ");
        assert!(!query.values.is_empty(), "values should not be empty, when inserting records");
        if !query.values.is_empty(){
            w.append("VALUES( ");
            let mut do_comma = false;
            for vo in &query.values{
                if do_comma{ w.commasp(); } else{do_comma=true;}
                self.build_operand(&mut w, query, vo);
            }
            w.append(") ");
        }
        if !query.enumerated_returns.is_empty() {
            if self.sql_options().contains(&SqlOption::SupportsReturningClause) {
                w.append("RETURNING ");
                let mut do_comma = false;
                for field in &query.enumerated_returns{
                    if do_comma{ w.commasp(); }else {do_comma = true;}
                    self.build_field(&mut w, query, field);
                }
            }
        }
        w.ln();
        w
    }

    ///TODO :complete this
    fn build_update(&self, query: &Query)->SqlFrag{
        println!("building update query");
        let mut w = SqlFrag::new(self.sql_options());
        w.append("UPDATE ");
        self.build_enumerated_fields(&mut w, query, &query.enumerated_fields); //TODO: add support for column_sql, fields, functions
        w.ln();
        let from_table = query.get_from_table();
        assert!(from_table.is_some(), "There should be table to update from");
        if from_table.is_some(){
            w.append(&from_table.unwrap().complete_name());
        }
        if !query.filters.is_empty() {
            w.ln_tab();
            w.append("WHERE ");
            self.build_filters(&mut w, query, &query.filters);
        }
        w
    }

    fn build_delete(&self, query: &Query)->SqlFrag{
        println!("building delete query");
        let mut w = SqlFrag::new(self.sql_options());
        w.append("DELETE FROM");
        let from_table = query.get_from_table();
        assert!(from_table.is_some(), "There should be table to delete from");
        if from_table.is_some(){
            w.append(&from_table.unwrap().complete_name());
        }
        if !query.filters.is_empty() {
            w.ln_tab();
            w.append("WHERE ");
            self.build_filters(&mut w, query, &query.filters);
        }
        w
    }

    fn sql_options(&self)->Vec<SqlOption>;

}

pub trait DatabaseDDL{
    //////////////////////////////////////////
    /// The following methods involves DDL(Data definition language) operation
    ////////////////////////////////////////

    /// create a database schema
    fn create_schema(&self, schema:&str);

    /// drop the database schema
    fn drop_schema(&self, schema:&str);

    /// create a database table based on the Model Definition
    fn create_table(&self, model:&Table);

    /// rename table, in the same schema
    fn rename_table(&self, table:&Table, new_tablename:String);

    /// drop table
    fn drop_table(&self, table:&Table);

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
    fn dbtype_to_rust_type(&self, db_type: &str)->(Vec<String>, String);
    
    fn rust_type_to_dbtype(&self, rust_type: &str, db_data_type:&str)->String;


    /// build a source code for the struct defined by this table
    ///(imports, imported_tables, source code)
    fn to_struct_source_code<'a>(&self, table:&'a Table, all_tables:&'a Vec<Table>)->(Vec<String>, Vec<&'a Table>, String){
        let mut w = Writer::new();
        //imported tables needed since we are partitioning the tables in schemas
        let mut imported_tables = Vec::new();
        //imports
        let mut imports:Vec<String> = Vec::new();
        for c in &table.columns{
            let (package, _) = self.dbtype_to_rust_type(&c.db_data_type);
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
