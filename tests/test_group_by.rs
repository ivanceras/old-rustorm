extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use uuid::Uuid;

use rustorm::query::Query;
use rustorm::query::QueryBuilder;
use rustorm::query::Equality;
use rustorm::dao::{Dao, IsDao};
use rustorm::pool::ManagedPool;
use rustorm::query::function::COUNT;
use rustorm::query::HasEquality;
use rustorm::query::join::ToJoin;
use rustorm::query::operand::ToOperand;
use rustorm::query::HasDirection;
use rustorm::query::builder::SELECT_ALL;


#[test]
fn test_group_by() {
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v8";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();

    let frag = SELECT_ALL().FROM(&"bazaar.product")
         .GROUP_BY(&["product.name", "product.created"])
    	 .build(db.as_ref());

    let expected = "
  SELECT *
     FROM bazaar.product
 GROUP BY name ,created 
 ".to_string();

    println!("actual:   {{\n{}}} [{}]", frag.sql, frag.sql.len());
    println!("expected: {{{}}} [{}]", expected, expected.len());
    assert_eq!(frag.sql.trim() , expected.trim());

}
