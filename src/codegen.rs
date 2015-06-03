use table::Table;
use writer::Writer;
use database::DatabaseDev;
use std::fs::File;
use std::io::Write;


/// configuration for generating code

pub struct Config{
    /// all tables should be residing in 1 module
    /// except when there are conflicting tables is different schema
    use_same_module_when_possible:bool,
    /// the module name to be used when it is possible to unify all tables in 1 module
    /// likely the project name
    base_module_name:Option<String>,
    /// use the schema as the module name
    /// i.e. bazaar::product, public::user
    use_schema_as_module:bool,
    /// include the has_one, has_many, extension Models
    include_references:bool,
    ///use the condense name of the has_many else, use the table name referred
    use_condensed_name:bool,
    
    ///generate the is table definition for each table
    generate_is_table:bool,
    /// append the table to struct modules
    append_is_table_to_structs:bool,//append the is_tables to the struct modules
    is_table_file:Option<String>,

    /// put everything in 1 module file
    /// this includes the is_tables, the structs (modularized in schema modules)
    use_one_file:bool,
    
    /// base directory for the generated content
    base_dir: String,
}

impl Config{
    
    pub fn default()->Self{
        Config{
            //schema will be broken into different module files
            use_same_module_when_possible:false,
            base_module_name:Some("gen".to_string()),
            use_schema_as_module:true,
            include_references:true,
            use_condensed_name:true,
            generate_is_table:true,
            append_is_table_to_structs:true,//append the is_tables to the struct modules
            is_table_file:Some("meta".to_string()),
            use_one_file:false,
            base_dir:".".to_string(),
        }
    }
    
    pub fn simple()->Self{
        Config{
            //schema will be broken into different module files
            use_same_module_when_possible:true,
            base_module_name:Some("gen".to_string()),
            use_schema_as_module:false,
            include_references:true,
            use_condensed_name:true,
            generate_is_table:true,
            append_is_table_to_structs:false,//append the is_tables to the struct modules
            is_table_file:Some("is_table".to_string()),
            use_one_file:false,
            base_dir:".".to_string(),
        }
    }
}

/// [TODO]: support compartmentalizing tables via schema
/// schema themselves becomes module that holds these tables
///
pub fn get_all_tables<T:DatabaseDev>(db_dev:&T)->Vec<Table>{
    let all_tables_names = db_dev.get_all_tables();
    let mut all_table_def:Vec<Table> = Vec::new();
    for (schema, table) in all_tables_names{
        println!("Extracted {}.{}", schema,table);
        let meta = db_dev.get_table_metadata(&schema, &table);
        all_table_def.push(meta);
    }
    all_table_def
}

/// determine if there is a conflicting table name that resides on different schema
pub fn has_conflicting_table_names(all_table:&Vec<Table>)->bool{
    let mut table_names = Vec::new();
    for t in all_table{
        if table_names.contains(&t.name){
            return true;
        }
        table_names.push(t.name.clone());
    }
    false
}

/// get the database schema
pub fn get_schemas(all_table:&Vec<Table>)->Vec<String>{
    let mut schema_names = Vec::new();
    for t in all_table{
        if !schema_names.contains(&t.schema){
            schema_names.push(t.schema.clone());
        }
    }
    schema_names.sort_by(|a, b| a.cmp(b));
    schema_names
}

/// get all tables with schema name
pub fn get_tables_in_schema(schema:&String, all_table:&Vec<Table>)->Vec<Table>{
    let mut tables = Vec::new();
    for t in all_table{
        if &t.schema == schema{
            tables.push(t.clone());//cloned the table here
        }
    }
    tables.sort_by(|a, b| a.name.cmp(&b.name));
    tables
}

/// this is the default config generation
pub fn generate_all<T:DatabaseDev>(db_dev:&T, config:&Config){
    let all_tables:Vec<Table> = get_all_tables(db_dev);
    let has_conflict = has_conflicting_table_names(&all_tables);
    let schemas = get_schemas(&all_tables);

    for schema in &schemas{
        let mut struct_w = Writer::new();//structs
        let mut is_table_w = Writer::new(); //is_tables
        let tables = get_tables_in_schema(schema, &all_tables);
        let (struct_imports, struct_src) = generate_structs_list(db_dev, &tables, &all_tables);
        struct_w.append(&build_imports(&struct_imports));
        for src in struct_src{
            struct_w.ln();
            struct_w.append(&src);
        }
        let struct_file = format!("{}/{}/{}.rs",config.base_dir,config.base_module_name.clone().unwrap(), schema); 
        save_to_file(&struct_file, &struct_w.src);
        let (is_table_imports, is_table_src) = generate_is_table_list_impl(&tables);
        
        is_table_w.append(&build_imports(&is_table_imports));
        for src in is_table_src{
            is_table_w.ln();
            is_table_w.ln();
            is_table_w.append(&src);
        }
        let is_table_file = format!("{}/{}/{}_{}.rs",config.base_dir,config.base_module_name.clone().unwrap(), schema, &config.is_table_file.clone().unwrap());
        is_table_w.append(&generate_fn_get_all_tables(&all_tables));
        save_to_file(&is_table_file, &is_table_w.src);
    }
}

/// returns the deduped imports and the struct source for each table
pub fn generate_structs_list<T:DatabaseDev>(db_dev:&T, tables:&Vec<Table>, all_table_def:&Vec<Table>)->(Vec<String>, Vec<String>){
    let mut struct_src = Vec::new();
    let mut struct_imports = Vec::new();
    for table in tables{
        let meta = db_dev.get_table_metadata(&table.schema, &table.name);
        println!("Generating for {}.{}", meta.schema,meta.name);
        let (imports, src) = db_dev.to_struct_source_code(&meta, all_table_def);
        for i in imports{
            if !struct_imports.contains(&i){
                struct_imports.push(i);
            }
        }
        struct_src.push(src);
    }
    (struct_imports, struct_src)
}

/// returns the deduped imports and the impl source for each table
pub fn generate_is_table_list_impl(tables:&Vec<Table>)->(Vec<String>, Vec<String>){
    let mut impl_src = Vec::new();
    let mut impl_imports = Vec::new();
    
    for table in tables{
        let (is_table_import, src) = generate_is_table_impl_code(table);
        for i in is_table_import{
            if !impl_imports.contains(&i){
                impl_imports.push(i);
            }
        }
        impl_src.push(src);
    }
    
    (impl_imports, impl_src)
}

fn generate_is_table_impl_code(table: &Table)->(Vec<String>, String){
    let mut w = Writer::new();
    let mut imports = Vec::new();
    imports.push("table::IsTable".to_string());
    imports.push("table::IsTable".to_string());
    imports.push("table::Column".to_string());
    imports.push("table::Foreign".to_string());
    imports.push("table::Table".to_string());
    imports.push(format!("gen::structs::{}", table.struct_name()));
    
    w.append("impl IsTable for ");
    w.append(&table.struct_name());
    w.append("{");
    w.ln();
    w.ln();
    w.tab();
    w.append("fn table()->Table{");
    w.ln();
    w.tab();
    w.append(&table.to_tabledef_source_code());
    w.ln();
    w.tab();
    w.append("}");
    w.ln();
    w.append("}");
    (imports, w.src)
}

/// build source code for the imports,
/// deduped to make sure no duplicated imports
fn build_imports(imports:&Vec<String>)->String{
    let mut import_clone = imports.clone();
    let mut w = Writer::new();
    import_clone.sort_by(|a, b| a.cmp(b));
    import_clone.dedup();
    for imp in import_clone{
        w.append("use ");
        w.append(&imp);
        w.append(";");
        w.ln();
    }
    w.src
}
fn generate_fn_get_all_tables(tables:&Vec<Table>)->String{
    let mut w = Writer::new();
    w.ln();
    w.ln();
    w.append("pub fn get_all_tables()->Vec<Table>{");
    w.ln();
    w.tab();
    w.append("vec![");
    for t in tables{
        w.ln();
        w.tabs(2);
        w.append(&t.struct_name());
        w.append("::table(),");
    }
    w.ln();
    w.tab();
    w.append("]");
    w.ln();
    w.append("}");
    w.src
}



fn save_to_file(filename: &str, content:&String){
    let mut file = match File::create(filename){
        Err(why) => panic!("couldn't create file {}", filename),
        Ok(mut file) => {
            match file.write_all(content.as_bytes()){
                Ok(x) => {println!("Saved to {}",filename);},
                Err(_) => {println!("There was error saving to file: {}",filename)}
            };
        },
    };
   
}
