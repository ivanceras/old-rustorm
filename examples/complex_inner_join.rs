extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use uuid::Uuid;

use rustorm::query::Query;
use rustorm::query::Equality;
use rustorm::dao::{Dao, IsDao};
use rustorm::pool::ManagedPool;

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
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();

    let mut query = Query::select_all();

    query.from_table("bazaar.product")
         .inner_join_table("bazaar.product_category",
                          "product_category.product_id",
                          "product.product_id")
         .inner_join_table("bazaar.category",
                          "category.category_id",
                          "product_category.category_id")
         .inner_join_table("product_photo",
                          "product.product_id",
                          "product_photo.product_id")
         .inner_join_table("bazaar.photo", "product_photo.photo_id", "photo.photo_id")
         .filter("product.name", Equality::EQ, &"GTX660 Ti videocard")
         .filter("category.name", Equality::EQ, &"Electronic")
         .group_by(vec!["category.name"])
         .having("count(*)", Equality::GT, &1)
         .asc("product.name")
         .desc("product.created");
    let frag = query.build(db.as_ref());

    let expected = "
   SELECT *
     FROM bazaar.product
          INNER JOIN bazaar.product_category\x20
          ON product_category.product_id = product.product_id\x20
          INNER JOIN bazaar.category\x20
          ON category.category_id = product_category.category_id\x20
          INNER JOIN product_photo\x20
          ON product.product_id = product_photo.product_id\x20
          INNER JOIN bazaar.photo\x20
          ON product_photo.photo_id = photo.photo_id\x20
    WHERE product.name = $1\x20
      AND category.name = $2\x20
 GROUP BY category.name\x20
   HAVING count(*) > $3\x20
 ORDER BY product.name ASC, product.created DESC".to_string();
    println!("actual:   {{\n{}}} [{}]", frag.sql, frag.sql.len());
    println!("expected: {{{}}} [{}]", expected, expected.len());
    assert!(frag.sql.trim() == expected.trim());

}
