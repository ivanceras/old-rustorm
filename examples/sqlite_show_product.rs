extern crate rustorm;

use rustorm::query::Query;
use rustorm::pool::ManagedPool;

fn main() {
    let url = "sqlite:///file.db";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();

    let dao = Query::select()
                  .columns(vec!["name"])
                  .from_table(&"product")
                  .retrieve(db.as_ref());

    println!("dao: {:#?}", dao);
}
