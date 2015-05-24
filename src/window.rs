use table::Table;

///directly corresponds to a column of a table
pub struct Field{
	pub name:String,
	pub column:String,
	pub data_type:String,
	pub description:Option<String>,
	pub help:Option<String>,
	
	///ordering of the fields
	pub seq_no:f32,
  	///should be same line or no
  	pub is_same_line:bool,
  	pub is_displayed:bool,
  	///whether or not this field is editable
  	pub is_readonly:bool,
	pub display_logic:Option<String>,
  	pub display_length:Option<String>, 
  	pub default_value:Option<String>,
}

pub struct Tab{
	pub name:String,
	pub is_extension:bool,
	pub is_has_one:bool,
	pub is_has_many:bool,
	pub is_direct:bool,
	pub description:String,
	///which table does it refer to
	pub table:String,
	pub schema:String,
	pub fields:Vec<Field>,
	///other children tabs
	pub tabs:Option<Vec<Tab>>,
}

///directly correspond to a table, no need for tabs
pub struct Window{
	///name of the window
	pub name:String,
	pub description:Option<String>,
	///main tab, must have at least 1
	pub tab:Tab,
}

/// build windows from a set of tables
/// 
pub fn extract_windows(tables:&Vec<Table>){
	
	for t in tables{
		if !t.is_linker_table(){
			println!("{}", t.name);
			for (col, has1) in t.referred_tables(tables){
				println!("\t has one: {}({}) {} condensed: {}",col.displayname(), col.name, has1.name, col.condense_name());
			}
			for ext in t.extension_tables(tables){
				println!("\t ext tab: {}", ext.name);
			}
			for (has_many,col) in t.referring_tables(tables){
				if !has_many.is_linker_table(){
					println!("\t has many direct: {}", has_many.name);
				}else{
					//println!("\t has many direct: {} <---- but is a linker table, so no!", has_many.name);
				}
			}
			for (has_many,linker) in t.indirect_referring_tables(tables){
				println!("\t has many: {} via {}",has_many.name, linker.name);
			}
		}
		else{
			println!("NOT a Window: {}", t.name);
		}
	}
}