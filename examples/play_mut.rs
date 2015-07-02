struct Pool{

	value: bool,
}

impl Pool{
	
	fn new()->Self{
		Pool{value:true}
	}
	
	fn set(&mut self){
		self.value = true;
	}
	
	fn unset(&mut self){
		self.value = false;
	}
}

fn main(){
	let mut pool = Pool::new();
	pool.set();
	
	pool.unset();
}