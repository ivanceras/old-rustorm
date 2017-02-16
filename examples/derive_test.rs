extern crate rustorm;
#[macro_use]
extern crate rustorm_derive;
use rustorm::dao::Dao;
use rustorm::dao::IsDao;


#[derive(IsDao)]
struct FrenchToast{
    cheese: String,
    age: u32,
}
#[derive(IsDao)]
struct Waffles{
    cone: u8,
    name: String,
}

fn main() {
    FrenchToast::hello_world();
    Waffles::hello_world();
}
