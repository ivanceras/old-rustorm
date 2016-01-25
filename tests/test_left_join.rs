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
fn test_left_join() {
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v8";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();

    let frag = SELECT_ALL().FROM(&"bazaar.product")
         .LEFT_JOIN("bazaar.product_category"
		 		.ON("product_category.product_id".EQ(&"product.product_id")
               .AND("product_category.product_id".EQ(&"product.product_id")))
		 )
         .LEFT_JOIN("bazaar.category"
                .ON("category.category_id".EQ(&"product_category.category_id")))
         .LEFT_JOIN("bazaar.product_photo"
		 		.ON("product.product_id".EQ(&"product_photo.product_id")))
         .LEFT_JOIN("bazaar.photo" 
		 		.ON("product_photo.photo_id".EQ(&"photo.photo_id")))
    	 .build(db.as_ref());

    let expected = "
SELECT *
     FROM bazaar.product
          LEFT JOIN bazaar.product_category
          ON ( product_category.product_id = product.product_id AND product_category.product_id = product.product_id  )
          LEFT JOIN bazaar.category
          ON category.category_id = product_category.category_id 
          LEFT JOIN bazaar.product_photo
          ON product.product_id = product_photo.product_id 
          LEFT JOIN bazaar.photo
          ON product_photo.photo_id = photo.photo_id
 ".to_string();

    println!("actual:   {{\n{}}} [{}]", frag.sql, frag.sql.len());
    println!("expected: {{{}}} [{}]", expected, expected.len());
    assert_eq!(frag.sql.trim() , expected.trim());

}
