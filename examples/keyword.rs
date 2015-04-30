macro_rules! SQL { 
		($($x:tt)*) => { stringify!($($x)*) }
}


macro_rules! all {
    ($t:tt $u: tt) => {
        fn get_all() -> &'static str {
           return format!("{}",stringify!($t $u));
        }
    }
}


macro_rules! create_sql { 
	($($x:tt)*) => { 
		fn SQL()->&'static str{
			return stringify!($($x)*);
		} 
	}
}

create_sql!(keys);

fn main() {
	println!("{}", SQL!(SELECT * FROM table));
}