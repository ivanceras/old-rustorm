macro_rules! create_function {
    // this macro takes an argument of "type" `ident`
    // the `ident` designator is used for variable/function names
    ($func_name:ident) => {
        // this macro creates a function with name `$func_name`
        fn $func_name() {
            // the stringify! macro converts an `ident` into a string
            println!("You called {}()",
                     stringify!($func_name))
        }
    }
}

macro_rules! keyword {
    ($func_name:ident) => {
        fn $func_name() -> String {
           return format!("{}",stringify!($func_name));
        }
    }
}

keyword!(SELECT);
keyword!(FROM);

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // the `expr` designator is used for expressions
    ($expression:expr) => {
        // stringify! will convert the expression *as it is* into a string
        println!("{} = {}",
                 stringify!($expression),
                 $expression)
    }
}

struct DAO{
	value:u64
}

impl DAO{
	fn class()->String{
		return format!("DAO");
	}
}
fn get_type<T>(t:T){

}


fn main() {
	let mut q =  SELECT();
	q.push_str(&format!(" {}", FROM()));
	println!("-->{}",q);
    foo();
    bar();
    let dao = DAO{value:1};
    get_type(dao);
    let class = DAO::class();
    println!("class: {}",class);
    print_result!(1u8 + 1);

    // remember that blocks are expressions
    print_result!({
        let x = 1u8;

        x * x + 2 * x - 1
    });
}

fn test(){
	println!("Welcome rustorm");
	let value = r#"
        This is a table
        SQL statements here!!...;
    "#;
    println!("value: {}", value);
	let sql = stringify!(
		"select * from table" lele jceasrasasldkl;,zxm,dzx,.dioquje87123891273872839
		asedasldkl asldks... 223423400323423^^
		);
	println!("sql: {}", sql);
	
	let table = "jedi";
	let mut f = format!("select * from {}", table);
	f.push_str(" where x = 1");
	println!("formatted: {}", f);
}

