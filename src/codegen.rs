use table::Table;
use writer::Writer;
use database::DatabaseDev;
use std::fs::File;
use std::io::Write;

pub fn generate_all_tables<T:DatabaseDev>(db_dev:T, filename:&str){
	let all_tables_names = db_dev.get_all_tables();
	let mut all_struct_src = Vec::new();
	let mut all_imports = Vec::new();
	let mut all_table_def:Vec<Table> = Vec::new();
	for (schema, table) in all_tables_names{
		println!("{}.{}", schema,table);
		let meta = db_dev.get_table_metadata(&schema, &table);
		all_table_def.push(meta);
	}
	
	
	for table in &all_table_def{
		let meta = db_dev.get_table_metadata(&table.schema, &table.name);
		let (imports, struct_src) = db_dev.to_source_code(&meta, &all_table_def);
		for i in imports{
			all_imports.push(i);
		}
		all_struct_src.push(struct_src);
	}
	
	let all_src = build_all(&mut all_imports, &all_struct_src);
	
	save_to_file(filename, &all_src);
}


fn build_all(imports:&mut Vec<String>, structs:&Vec<String>)->String{
	let mut all_src = String::new();
	let import_src = build_imports(imports);
	all_src.push_str(&import_src);
	for s in structs {
		all_src.push_str(&s);
	}
	all_src
}

fn build_imports(imports:&mut Vec<String>)->String{
	let mut w = Writer::new();
	imports.sort_by(|a, b| a.cmp(b));
	imports.dedup();
	for imp in imports{
		w.append("use ");
		w.append(&imp);
		w.append(";");
		w.ln();
	}
	w.src
}

fn save_to_file(filename: &str, content:&String){
	let mut file = match File::create(filename){
		Err(why) => panic!("couldn't create file {}", filename),
        Ok(file) => file,
	};
	file.write_all(content.as_bytes());
	println!("Saved to {}",filename);
}
