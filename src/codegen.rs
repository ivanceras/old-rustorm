use table::Table;
use writer::Writer;
use database::DatabaseDev;
use std::fs::File;
use std::fs;
use std::io::Write;


/// configuration for generating code

pub struct Config{
    /// the module name to be used when it is possible to unify all tables in 1 module
    /// likely the project name
    pub base_module:Option<String>,
    /// include the has_one, has_many, extension Models
    pub include_table_references:bool,
    ///use the condense name of the has_many else, use the table name referred
    pub use_condensed_name:bool,
    
    ///generate the is table definition for each table
    pub generate_table_meta:bool,

    /// base directory for the generated content
    pub base_dir: String,
}

impl Config{
    
    pub fn default()->Self{
        Config{
            base_module:Some("gen".to_string()),
            include_table_references:true,
            use_condensed_name:true,
            generate_table_meta:true,
            base_dir:"./src".to_string(),
        }
    }
    
    fn table_module(&self, table:&Table)->String{
        let parent = self.module(&table.schema);
        format!("{}::{}",parent,table.struct_name())
    }
    
    fn module_dir(&self, schema:&String)->String{
        let base_module = self.base_module.clone();
        match base_module{
            Some(x) => format!("{}/{}/{}",self.base_dir, x, schema),
            None => format!("{}/{}",self.base_dir, schema)
        }
    }
    fn module_base_dir(&self)->String{
        let base_module = self.base_module.clone();
        match base_module{
            Some(x) => format!("{}/{}",self.base_dir, x),
            None => format!("./{}",self.base_dir)
        }
    }
    
    fn module(&self, schema:&String)->String{
        let base_module = self.base_module.clone();
        match base_module{
            Some(x) => format!("{}::{}", x, schema),
            None => format!("{}", schema)
        }
    }
    
}

///
/// retrieve all the table definition in the database
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
pub fn get_tables_in_schema<'a>(schema:&String, all_table:&'a Vec<Table>)->Vec<&'a Table>{
    let mut tables = Vec::new();
    for t in all_table{
        if &t.schema == schema{
            tables.push(t);//cloned the table here
        }
    }
    tables.sort_by(|a, b| a.name.cmp(&b.name));
    tables
}
/// this is the default config generation
/// FIXME need to compartmentalize each
/// 1. structs
/// 2. meta
/// mod.rs

pub fn generate_all<T:DatabaseDev>(db_dev:&T, config:&Config){
    let all_tables:Vec<Table> = get_all_tables(db_dev);
    for table in &all_tables{
        generate_table(db_dev, config, table, &all_tables);
    }
    generate_mod_per_schema(&config, &all_tables);
    generate_mod_rs(&config, &all_tables);
}

/// the gernaration of tables should be placed on their respective directory
/// base_mod::schema::table.rs
fn generate_table<T:DatabaseDev>(db_dev:&T, config:&Config, table:&Table, all_tables:&Vec<Table>){
    let mut w = Writer::new();
    let (struct_imports, imported_tables, struct_src) = db_dev.to_struct_source_code(&table, all_tables);
    let (dao_imports, dao_src) = generate_dao_conversion_code(table, all_tables);
    let (meta_imports, meta_src) = generate_meta_code(table);
    let static_columns = generate_static_column_names(table);
    
    for i in struct_imports{
        w.append(&format!("use {};",i));
        w.ln();
    }
    
    for it in imported_tables{
        if it != table{
            w.append(&format!("use {};", config.table_module(it)));
            w.ln();
        }
    }
    for i in dao_imports{
        w.append(&format!("use {};",i));
        w.ln();
    }
    for i in meta_imports{
        w.append(&format!("use {};",i));
        w.ln();
    }
    w.ln();
    w.ln();
    w.append(&struct_src);
    w.ln();
    w.ln();
    w.append(&static_columns);
    w.ln();
    w.ln();
    w.append(&dao_src);
    w.ln();
    w.ln();
    w.append(&meta_src);
    
    let module_dir = config.module_dir(&table.schema);
    fs::create_dir_all(&module_dir);
    let file = format!("{}/{}.rs", &module_dir, table.name);
    save_to_file(&file, &w.src);
}


fn generate_mod_per_schema(config:&Config, all_tables:&Vec<Table>){
    let schemas = get_schemas(all_tables);
    for schema in schemas{
        let mut w = Writer::new();
         let module_dir = config.module_dir(&schema);
         let tables = get_tables_in_schema(&schema, all_tables);
         for table in &tables{
            w.append(&format!("pub mod {};",table.name));
            w.ln();
         }
         for table in &tables{
            //re-export structs
            w.append(&format!("pub use self::{}::{};", table.name, table.struct_name()));
            w.ln();
         }
         let mod_file = format!("{}/mod.rs", module_dir);
         save_to_file(&mod_file, &w.src);
    }
}


fn generate_mod_rs(config:&Config, all_tables:&Vec<Table>){
    let mut mod_src = Writer::new();
    let schemas = get_schemas(&all_tables);
    for schema in &schemas{
        mod_src.append("pub mod ");
        mod_src.append(schema);
        mod_src.append(";");
        mod_src.ln();
    }
    mod_src.append("use rustorm::table::Table;");
    mod_src.ln();
    mod_src.append("use rustorm::table::IsTable;");
    mod_src.ln();
    mod_src.ln();
    for table in all_tables{
        let table_mod = format!("use {};", config.table_module(table));
        mod_src.append(&table_mod);
        mod_src.ln();
    }
    let mod_file = format!("{}/mod.rs",config.module_base_dir());
    let all_table_fn = &generate_fn_get_all_tables(&all_tables);
    mod_src.append(all_table_fn);
    save_to_file(&mod_file, &mod_src.src);

}



fn generate_meta_code(table: &Table)->(Vec<String>, String){
    let mut w = Writer::new();
    let mut imports = Vec::new();
    imports.push("rustorm::table::IsTable".to_string());
    imports.push("rustorm::table::Column".to_string());
    imports.push("rustorm::table::Foreign".to_string());
    imports.push("rustorm::table::Table".to_string());
    
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


fn generate_static_column_names(table: &Table)->String{
    let mut w = Writer::new();
    w.comment(" Generated columns for easier development of dynamic queries without sacrificing wrong spelling of column names");
    for column in &table.columns{
        w.ln();
        w.append("pub static ");
        w.append(&column.name);
        w.append(": &'static str = ");
        w.append(&format!("\"{}.{}\"", table.name, column.name));
        w.append(";");
    }
    w.src
}

/// TODO: if column names begins with the tablename_, then put this value to the column name hash map
/// example: product_name, value will be copied to name
/// test if product_name is not a column, split with `_`, then check if first splinter is the tablename, then the check if the 2nd splinter if 
/// a column of this table, then set that column with the value of the original name 
fn generate_dao_conversion_code(table: &Table, all_tables:&Vec<Table>)->(Vec<String>, String){
    let mut w = Writer::new();
    let mut imports = Vec::new();
    imports.push("rustorm::dao::Dao".to_string());
    imports.push("rustorm::dao::IsDao".to_string());
    imports.push("rustorm::dao::DaoResult".to_string());
    imports.push("std::collections::BTreeMap".to_string());
    
    w.ln();
    w.append("impl IsDao for ");
    w.append(&table.struct_name());
    w.append("{");
    w.ln_tab();
    w.append("fn from_dao(dao:&Dao)->Self{");
    w.ln_tabs(2);
    w.append(&table.struct_name());
    w.append("{");
    for c in &table.columns{
        w.ln_tabs(3);
        w.append(&c.name);
        w.append(": ");
        if c.not_null{
            w.append("dao.get");
        }else{
            w.append("dao.get_opt");
        }
        w.append("(\"");
        w.append(&c.name);
        w.append("\")");
        w.comma();
    }
    let referenced_tables = table.get_all_referenced_table(all_tables);
    for ref_table in referenced_tables{
        let member_name = ref_table.member_name(table);
        w.ln_tabs(3);
        w.append(&member_name);
        w.append(": ");
        if ref_table.is_has_one{
            w.append("None");
        }
        if ref_table.is_ext{
            w.append("None");
        }
        if ref_table.is_has_many{
            w.append("vec![]");
        }
        w.comma();
    }
    w.ln_tabs(2);
    w.append("}");
    w.ln_tab();
    w.append("}");
    w.ln();
    w.append("}");
    (imports, w.src)
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
   match File::create(filename){
        Err(_) => panic!("couldn't create file {}", filename),
        Ok(mut file) => {
            match file.write_all(content.as_bytes()){
                Ok(_) => {println!("Saved to {}",filename);},
                Err(_) => {println!("There was error saving to file: {}",filename)}
            };
        },
    };
   
}
