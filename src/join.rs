pub enum JoinType{
		CROSS,
		INNER,
		OUTER,
}

pub enum Modifier{
		LEFT,
		RIGHT,
		FULL,
}


pub struct Join{
	pub modifier:Option<Modifier>,
	pub join_type:JoinType,
	pub table:String,
	pub column1:Vec<String>,
	pub column2:Vec<String>
}
