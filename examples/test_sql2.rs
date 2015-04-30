
pub struct SQL<'a>{
	keywords:Vec<&'static str>,
}

macro_rules! keyword {
    ($keyword: ident) => {
        fn $keyword(&'a mut self) -> &'a mut Self{
           self.keywords.push(stringify!($keyword));
           self
        }
    }
}

macro_rules! all_keywords{
	() => {
		keyword!(SELECT);
		keyword!(FROM);
		keyword!(TABLE);
		keyword!(MAX);
	}
}

fn MAX<'a>()->&'a mut SQL<'a>{
	//let sql = SQL::new();
	//SQL::new().field()
}

impl <'a> SQL<'a>{
	pub fn new()->&'a SQL{
		SQL{keywords:Vec::new()}
	}
	
	all_keywords!();
	
	fn to_string(&mut self)->String{
		let mut s = String::new();
		for k in &self.keywords{
			s.push_str(k);
		}
		s
	}
	
	fn field(&'a mut self)->&'a mut Self{
		self.keywords.push("field");
		self
	}
}

fn main(){
	let mut sql = SQL::new();
	let v = sql.field().SELECT().FROM();
	println!("v: {}",v.to_string());
}