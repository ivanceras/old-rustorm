pub struct Fluent;

pub struct Select;

pub struct SelectAfterWhere;

pub struct SelectAfterJoin;

impl Fluent{
	fn SELECT()->Select{
		unimplemented!()	
	}
}

impl Select{
	fn COUNT(&self)->&Select{
		self	
	}
	fn FROM(&self, s: &str)->&Select{
		self	
	}
	fn LEFT_JOIN(&self, s: &str)->SelectAfterJoin{
		unimplemented!()
	}
	fn WHERE(&self)->&SelectAfterWhere{
		unimplemented!()
	}
}

impl SelectAfterWhere{
	
	fn AND(&self)->&SelectAfterWhere{
		unimplemented!()
	}
}


fn main(){

	let q = Fluent::SELECT();
	q.COUNT()
		.FROM("bazaar.product")
		.WHERE()
		.AND()
		;

}
