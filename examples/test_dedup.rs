fn main(){
	let mut v = vec!["hi", "hello", "hi", "world"];
	v.sort_by(|a, b| a.cmp(b));//sort first before deduplicating
	v.dedup();
	for i in &v{
		println!("{}", i)
	}
	
	let mut vec = vec![1, 2, 2, 3, 2];
	vec.sort_by(|a, b| a.cmp(b));
	vec.dedup();
	for i in &vec{
		println!("{}", i)
	}
}