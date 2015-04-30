pub struct SQL{
	keywords:Vec<String>,
	parameters:Vec<String>
}

///
/// This creates a method for each corresponding SQL keywords
/// 
/// i.e created_method(SELECT)
////
/// fn SELECT(mut self){
///     self.push("SELECT");
/// }
///
macro_rules! create_method{
    ($keyword: ident) => {
        pub fn $keyword(mut self) -> Self{
           self.push_keyword((stringify!($keyword).to_string()))
        }
    }
}

macro_rules! create_all_keywords_method{
	() => {
		create_method!(SELECT);
		create_method!(FROM);
		create_method!(TABLE);
		create_method!(MAX);
		create_method!(MIN);
		create_method!(WHERE);
		create_method!(EQUAL_TO);
	}
}

pub fn MAX(sql:SQL)->SQL{
	SQL::new().MAX()
		.open()
		.push_keyword(sql.get_string())
		.close()
}

pub fn FLD(column:&str)->SQL{
	let column = column.to_string();
	SQL::new().push_keyword(column)
}

pub fn PARAM(param:&str)->SQL{
	SQL::new().push_param(param.to_string())
}

pub fn FLDS(columns:&[&str])->SQL{
	let mut sql = SQL::new();
	let mut do_comma = false; 
	for c in columns{
		if do_comma {sql = sql.comma();}else{ do_comma = true;}
		sql = sql.push_keyword(c.to_string()).space();
	}
	sql
}

pub fn SELECT(arg:SQL)->SQL{
	SQL::new().SELECT()
	.space()
	.push_keyword(arg.get_string())
}




pub fn FN(fnc:String)->SQL{
	SQL::new().push_keyword(fnc)
}

pub fn MIN(sql:SQL)->SQL{
	SQL::new().MAX()
}

impl  SQL{
	pub fn new()->Self{
		SQL{keywords:Vec::new(), parameters:Vec::new()}
	}
	
	create_all_keywords_method!();
	
	pub fn get_string(&self)->String{
		let mut s = String::new();
		for k in &self.keywords{
			s.push_str(k);
		}
		s
	}
	
	pub fn get_parameters(self)->Vec<String>{
		self.parameters
	}
	
	fn push_keyword(mut self, keyword:String)->Self{
		self.keywords.push(keyword);
		self
	}
	
	fn space(mut self)->Self{
		self.push_keyword(format!(" "))
	}
	
	fn comma(mut self)->Self{
		self.push_keyword(format!(","))
	}
	
	/// openparenthesis
	fn open(mut self)->Self{
		self.push_keyword(format!("("))
	}
	/// close parenthesis
	fn close(mut self)->Self{
		self.push_keyword(format!(")"))
	}
	
	pub fn push_str(mut self, keyword:String){
		self.push_keyword(keyword);
	}
	
	pub fn push_param(mut self, param:String)->Self{
		self.parameters.push(param);
		self
	}
	
	fn push_all_keyword(mut self, keyword:&[String])->Self{
		for k in keyword{
			self = self.push_keyword(k.clone());
		}
		self
	}
}

