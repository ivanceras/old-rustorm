
pub struct SQL{
	keywords:Vec<&'static str>,
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
        fn $keyword(mut self) -> Self{
           self.push_keyword(stringify!($keyword))
        }
    }
}

macro_rules! create_all_keywords_method{
	() => {
		create_method!(SELECT);
		create_method!(FROM);
		create_method!(TABLE);
		create_method!(MAX);
	}
}

fn MAX()->SQL{
	SQL::new().MAX()
}

impl  SQL{
	pub fn new()->Self{
		SQL{keywords:Vec::new()}
	}
	
	create_all_keywords_method!();
	
	fn to_string(&mut self)->String{
		let mut s = String::new();
		for k in &self.keywords{
			s.push_str(k);
		}
		s
	}
	
	fn push_keyword(mut self, keyword:&'static str)->Self{
		self.keywords.push(keyword);
		self
	}
}

fn main(){
	let mut sql = SQL::new();
	let mut v = sql.push_keyword("This are keywords").SELECT().FROM().TABLE();
	println!("v: {}",v.to_string());
	println!("{}",MAX().to_string());
}