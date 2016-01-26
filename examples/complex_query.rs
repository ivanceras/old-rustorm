extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use uuid::Uuid;

use rustorm::query::{Query,QueryBuilder};
use rustorm::query::Equality;
use rustorm::dao::{Dao, IsDao};
use rustorm::pool::ManagedPool;
use rustorm::query::HasEquality;
use rustorm::query::function::COUNT;
use rustorm::query::ToTableName;
use rustorm::query::HasDirection;
use rustorm::query::order::ToOrder;
use rustorm::query::join::ToJoin;
use rustorm::query::operand::ToOperand;
use rustorm::query::builder::SELECT_ALL;

#[derive(Debug, Clone)]
pub struct Photo {
    pub photo_id: Uuid,
    pub url: Option<String>,
}

impl IsDao for Photo{
    fn from_dao(dao: &Dao) -> Self {
        Photo {
            photo_id: dao.get("photo_id"),
            url: dao.get_opt("url"),
        }
    }

    fn to_dao(&self) -> Dao {
        let mut dao = Dao::new();
        dao.set("photo_id", &self.photo_id);
        match self.url {
            Some(ref _value) => dao.set("url", _value),
            None => dao.set_null("url"),
        }
        dao
    }
}

fn main() {
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v8";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();

    let frag = SELECT_ALL()
         .FROM(&"bazaar.product")
         .LEFT_JOIN("bazaar.product_category"
		 		.ON("product_category.product_id".EQ(&"product.product_id")
		 			.AND("product_category.product_id".EQ(&"product.product_id"))
				)
			)
         .LEFT_JOIN("bazaar.category"
		 		.ON("category.category_id".EQ(&"product_category.category_id")))

         .LEFT_JOIN("product_photo"
                .ON("product.product_id".EQ(&"product_photo.product_id")))
         .LEFT_JOIN("bazaar.photo"
		 	    .ON("product_photo.photo_id".EQ(&"photo.photo_id")))
         .WHERE(
		 	"product.name".EQ(&"GTX660 Ti videocard".to_owned())
         	.AND("category.name".EQ(&"Electronic".to_owned()))
			)
         .GROUP_BY(&["category.name","category.id"])
         .HAVING(COUNT(&"*").GT(&1))
         .ORDER_BY(&["product.name".ASC()])
         .build(db.as_ref());

    let expected = "
	   SELECT *
     FROM bazaar.product
          LEFT JOIN bazaar.product_category
          ON ( product_category.product_id = product.product_id AND product_category.product_id = product.product_id  )
          LEFT JOIN bazaar.category
          ON category.category_id = product_category.category_id 
          LEFT JOIN product_photo
          ON product.product_id = product_photo.product_id 
          LEFT JOIN bazaar.photo
          ON product_photo.photo_id = photo.photo_id 
    WHERE ( product.name = $1  AND category.name = $2   )
 GROUP BY category.name ,category.id 
   HAVING  COUNT(*) > $3  
 ORDER BY product.name ASC"
 .to_string();
    println!("actual:   {{\n{}}} [{}]", frag.sql, frag.sql.len());
    println!("expected: {{{}}} [{}]", expected, expected.len());
    assert!(frag.sql.trim() == expected.trim());

}
