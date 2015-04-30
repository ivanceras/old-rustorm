pub struct Column{
	pub name:String,
	pub data_type:String,
	pub comment:String,
}

pub struct Table{
	
	//which schema this belongs
	pub schema:String,

	//the table name
	pub name:String,
	
	//the parent table of this table when inheriting (>= postgresql 9.3)
	pub parent_table:String,
	
	//what are the other table that inherits this
	pub sub_table:Vec<String>,
	
	//comment of this table
	pub comment:String,
	
	pub columns:Vec<Column>,
	
	//primary columns of this table
	pub primary:Vec<String>,
	
	//unique columns of this table
	pub unique:Vec<String>,
	
	//when refering a 1:1 relationship to another table
	pub has_one:Vec<String>,
	//local column of this model
	pub has_one_local_column:Vec<String>,
	// referred column from the referred table
	pub has_one_referenced_column:Vec<String>,
	
	//when refering a 1:M relationshipt to another table
	pub has_many:Vec<String>,
	//local column
	pub has_many_referenced_column:Vec<String>,
	//referred column from the referred table
	pub has_many_local_column:Vec<String> 
	
}