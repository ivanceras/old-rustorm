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
use rustorm::query::builder::UPDATE;
use rustorm::dao::ToValue;



#[test]
fn test_update() {
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v8";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();

    let frag = UPDATE(&"bazaar.product")
            .SET("name", &"GoPro".to_owned())
            .SET("description", &"A nice camera".to_owned())
            .SET("active", &true)
            .WHERE("product_id".EQ(&10001))
    	 .build(db.as_ref());

    let expected = "
 UPDATE bazaar.product
      SET name = $1 , description = $2 , active = $3 
    WHERE product_id = $4  
 ".to_string();

    println!("actual:   {{\n{}}} [{}]", frag.sql, frag.sql.len());
    println!("expected: {{{}}} [{}]", expected, expected.len());
    assert_eq!(frag.sql.trim() , expected.trim());

}
