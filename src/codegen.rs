use table::Table;
use writer::Writer;
use database::DatabaseDev;
use std::fs::File;
use std::io::Write;

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

pub fn generate_all<T:DatabaseDev>(db_dev:&T, struct_file:&str, impl_file:&str){
    let all_tables:Vec<Table> = get_all_tables(db_dev);
    let (struct_imports, struct_src) = generate_all_structs(db_dev, &all_tables);
    
    let mut struct_w = Writer::new();
    struct_w.append(&build_imports(&struct_imports));
    for src in struct_src{
        struct_w.ln();
        struct_w.append(&src);
    }
    save_to_file(struct_file, &struct_w.src);
    
    let (impl_imports, impl_src) = generate_all_is_table_impl(&all_tables);
    let mut impl_w = Writer::new();
    impl_w.append(&build_imports(&impl_imports));
    for src in impl_src{
        impl_w.ln();
        impl_w.ln();
        impl_w.append(&src);
    }
    impl_w.append(&generate_fn_get_all_tables(&all_tables));
    save_to_file(impl_file, &impl_w.src);
    
}

/// returns the deduped imports and the struct source for each table
pub fn generate_all_structs<T:DatabaseDev>(db_dev:&T, all_table_def:&Vec<Table>)->(Vec<String>, Vec<String>){
    let mut struct_src = Vec::new();
    let mut struct_imports = Vec::new();
    for table in all_table_def{
        let meta = db_dev.get_table_metadata(&table.schema, &table.name);
        println!("Generating for {}.{}", meta.schema,meta.name);
        let (imports, src) = db_dev.to_struct_source_code(&meta, &all_table_def);
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
pub fn generate_all_is_table_impl(all_table_def:&Vec<Table>)->(Vec<String>, Vec<String>){
    let mut impl_src = Vec::new();
    let mut impl_imports = Vec::new();
    
    for table in all_table_def{
        let (is_table_import, src) = generate_is_table_impl(table);
        for i in is_table_import{
            if !impl_imports.contains(&i){
                impl_imports.push(i);
            }
        }
        impl_src.push(src);
    }
    
    (impl_imports, impl_src)
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

fn generate_is_table_impl(table: &Table)->(Vec<String>, String){
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
