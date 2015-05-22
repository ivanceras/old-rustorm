use std::fmt;

#[derive(Debug, Clone)]
pub struct Foreign{
	pub schema:String,
	pub table:String,
	pub column:String,
}

#[derive(Debug)]
pub struct Column{
	pub name:String,
	pub data_type:String,
	pub is_primary:bool,
	pub is_unique:bool,
	pub default:Option<String>,
	pub comment:Option<String>,
	pub not_null:bool,
	pub foreign:Option<Foreign>,
	///determines if the column is inherited from the parent table
	pub is_inherited:bool,
}

impl Column{

	//some column names may be a rust reserve keyword, so have to correct them
	pub fn get_corrected_name(&self)->String{
		if Self::is_keyword(&self.name){
		    return format!("{}_",self.name);
		}
		self.name.clone()
	}

	fn is_keyword(str:&str)->bool{
		let keyword = ["type", "yield"];
		keyword.contains(&str)
	}

}


impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug)]
pub struct Table{

	///which schema this belongs
	pub schema:String,

	///the table name
	pub name:String,

	///the parent table of this table when inheriting (>= postgresql 9.3)
	pub parent_table:Option<String>,

	///what are the other table that inherits this
	pub sub_table:Option<Vec<String>>,

	///comment of this table
	pub comment:Option<String>,

	///columns of this table
	pub columns:Vec<Column>,

}
impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}


impl Table{

	/// return all the primary columns of this table
	pub fn get_primary_columns(&self)->Vec<&Column>{
		let mut primary_columns = Vec::new();
		for c in &self.columns{
			if c.is_primary{
				primary_columns.push(c);
			}
		}
		primary_columns
	}
	
	/// return all the columns of this table excluding the inherited columns
	pub fn get_uninherited_columns(&self)->Vec<&Column>{
		let mut included = Vec::new();
		let mut uninherited_columns = Vec::new();
		for c in &self.columns{
			if !c.is_inherited && !included.contains(&&c.name){
				uninherited_columns.push(c);
				included.push(&c.name);
			}
		}
		uninherited_columns.sort_by(|a, b| a.name.cmp(&b.name));
		uninherited_columns
	}

	/// return all the inherited columns
	pub fn get_inherited_columns(&self)->Vec<&Column>{
		let mut included = Vec::new();
		let mut inherited_columns = Vec::new();
		for c in &self.columns{
			if c.is_inherited && !included.contains(&&c.name){
				inherited_columns.push(c);
				included.push(&c.name);
			}
		}
		inherited_columns.sort_by(|a, b| a.name.cmp(&b.name));
		inherited_columns
	}

	/// check to see if the column is a primary or not
	/// the Column.is_primary property is not reliable since it also list down the foreign key
	/// which makes it 2 entries in the table
	pub fn is_primary(&self, column_name:&str)->bool{
		for p in self.get_primary_columns(){
			if p.name == column_name {
				return true;
			}
		}
		false
	}

	/// return all the unique keys of this table
	pub fn get_unique_columns(&self)->Vec<&Column>{
		let mut unique_columns = Vec::new();
		for c in &self.columns{
			if c.is_unique{
				unique_columns.push(c);
			}
		}
		unique_columns
	}

	pub fn get_foreign_columns(&self)->Vec<&Column>{
		let mut columns = Vec::new();
		for c in &self.columns{
			if c.foreign.is_some(){
				columns.push(c);
			}
		}
		columns
	}

	/// get the table definition using the table name from an array of table object
	pub fn get_table<'a>(table_name:&str, tables: &'a Vec<Table>)->Option<&'a Table>{
		for t in tables{
			if t.name == table_name{
				return Some(t);
			}
		}
		None
	}



	/// get all the tables that is referred by this table
	/// get has_one
	pub fn get_referred_tables<'a>(&'a self, tables:&'a Vec<Table>)->Vec<(&'a Column, &'a Table)>{
		let mut referred_tables = Vec::new();
		for c in &self.columns{
			if c.foreign.is_some(){
				let ftable_name = &c.foreign.clone().unwrap().table;
				let ftable = Self::get_table(ftable_name, tables).unwrap();
				referred_tables.push((c, ftable));
			}
		}
		referred_tables
	}

	/// get all other tables that is refering to this table
	/// when any column of a table refers to this table
	/// get_has_many
	pub fn get_referring_tables<'a>(&self, tables: &'a Vec<Table>)->Vec<(&'a Table, &'a Column)>{
		let mut referring = Vec::new();
		for t in tables{
			for c in &t.columns{
				if c.foreign.is_some(){
					if &self.name == &c.foreign.clone().unwrap().table{
						referring.push((t, c));
					}
				}
			}
		}
		referring
	}
	
	/// using the get_indirect_referring_tables, we can get the linker tables
	/// this will be used as too not be included in the list of hasOne
	pub fn get_linker_tables<'a>(&self, tables: &'a Vec<Table>)->Vec<&'a Table>{
		let mut linker_tables = Vec::new();
		for ( _, linker) in self.get_indirect_referring_tables(tables){
			linker_tables.push(linker);
		}
		linker_tables
	}
	
	pub fn in_tables<'a>(table:&Table, tables: &'a Vec<&Table>)->bool{
		for tb in tables{
			if table.name == tb.name && table.schema == tb.schema{
				return true;
			}
		}
		false
	}

	/// when there is a linker table, bypass the 1:1 relation to the linker table
	/// then create a 1:M relation to the other linked table
	/// Algorithmn: determine whether a table is a linker then get the other linked table
    ///		*get all the referring table
	///		*for each table that refer to this table
	///		*if there are only 2 columns and is both primary
	///			and foreign key at the same time
	/// 		and 1 of which refer to the primary column of this table
	/// 	* then the other table that is refered is the indirect referring table
	/// returns the table that is indirectly referring to this table and its linker table
	pub fn get_indirect_referring_tables<'a>(&self, tables: &'a Vec<Table>)->Vec<(&'a Table, &'a Table)>{
		let mut indirect_referring_tables = Vec::new();
		for (rt, column) in self.get_referring_tables(tables){
			let rt_pk = rt.get_primary_columns();
			let rt_fk = rt.get_foreign_columns();
			let rt_uc = rt.get_uninherited_columns();
			if rt_pk.len() == 2 && rt_fk.len() == 2 && rt_uc.len() == 2 {
				println!("{} is a candidate linker table for {}", rt.name, self.name);
				let ref_tables = rt.get_referred_tables(tables);
				let (_, t0) = ref_tables[0];
				let (_, t1) = ref_tables[1];
				let mut other_table;
				if self.name == t0.name && self.schema == t0.schema{
					other_table = t1;
				}
				else{
					other_table = t0;
				}
				let mut cnt = 0;
				for fk in &rt_fk{
					if self.is_foreign_column_refer_to_primary_of_this_table(fk){
						cnt += 1;
					}
					if other_table.is_foreign_column_refer_to_primary_of_this_table(fk){
						cnt += 1;
					}
				}
				
				if cnt == 2{
					indirect_referring_tables.push((other_table, rt))
				}
			}
		}
		indirect_referring_tables
	}
	
	fn is_foreign_column_refer_to_primary_of_this_table(&self, fk:&Column)->bool{
		if fk.foreign.is_some(){
			let foreign = fk.foreign.clone().unwrap();
			let table = foreign.table;
			let schema = foreign.schema;
			let column = foreign.column;
			if self.name == table && self.schema == schema && self.is_primary(&column){
				return true;
			}
		}
		false
	}
	
	/// get referring tables, and check if primary columns of these referring table
	/// is the same set of the primary columns of this table
	/// it is just an extension table
	pub fn get_extension_tables<'a>(&self, tables: &'a Vec<Table>)->Vec<&'a Table>{
		let mut extension_tables = Vec::new();
		for (rt, _) in self.get_referring_tables(tables){
			let rt_pk = rt.get_primary_columns();
			let rt_fk = rt.get_foreign_columns();
			//if the referring tables's foreign columns are also its primary columns
			//that refer to the primary columns of this table
			//then that table is just an extension table of this table
			if Self::same_content(&rt_pk, &rt_fk){
				//if all fk refer to the primary of this table
				if self.are_these_foreign_column_refer_to_primary_of_this_table(&rt_fk){
					extension_tables.push(rt);
				}
			}
		}
		extension_tables
	}

	fn are_these_foreign_column_refer_to_primary_of_this_table(&self, rt_fk:&Vec<&Column>)->bool{
		let mut cnt = 0;
		for fk in rt_fk{
			if self.is_foreign_column_refer_to_primary_of_this_table(fk){
				cnt += 1;
			}
		}
		cnt == rt_fk.len()
	}


	/// checks if both arrays contains the same columns
	fn same_content(c1:&Vec<&Column>, c2:&Vec<&Column>)->bool{
		if c1.len() != c2.len() {
			return false;//early return
		}
		let mut column_names1 = Vec::new();
		for c in c1{
			column_names1.push(c.name.clone());
		}
		let mut column_names2 = Vec::new();
		for c in c2{
			column_names2.push(c.name.clone());
		}
		let mut cnt1 = 0;
		for d in &column_names1{
			if column_names2.contains(d){
				cnt1 += 1;
			}
		}
		let mut cnt2 = 0;
		for d in &column_names2{
			if column_names1.contains(d){
				cnt2 += 1;
			}
		}
		cnt1 > 0 && cnt2 > 0 && cnt1 == cnt2
	}

	/// capitalize the first later, if there is underscore remove it then capitalize the next letter
	pub fn get_struct_name(&self)->String{
		let mut struct_name = String::new();
		for i in self.name.split('_'){
				struct_name.push_str(&capitalize(i));
		}
		struct_name
	}



}

fn capitalize(str:&str)->String{
 	str.chars().take(1)
 		.flat_map(char::to_uppercase)
        .chain(str.chars().skip(1))
        .collect()
}
