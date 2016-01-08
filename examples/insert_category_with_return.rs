extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use uuid::Uuid;
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;

use rustorm::query::Query;
use rustorm::dao::{Dao, IsDao};
use rustorm::pool::ManagedPool;
use rustorm::table::{IsTable, Table};

#[derive(Debug, Clone)]
struct Category {
    pub category_id: Uuid,
    pub name: Option<String>,
    pub active: bool,
    pub client_id: Option<Uuid>,
    pub created: DateTime<UTC>,
    pub created_by: Option<Uuid>,
    pub description: Option<String>,
    pub help: Option<String>,
    pub organization_id: Option<Uuid>,
    pub priority: Option<f64>,
    pub updated: DateTime<UTC>,
    pub updated_by: Option<Uuid>,
}

impl IsDao for Category{
    fn from_dao(dao: &Dao) -> Self {
        Category {
            organization_id: dao.get_opt("organization_id"),
            client_id: dao.get_opt("client_id"),
            created: dao.get("created"),
            created_by: dao.get_opt("created_by"),
            updated: dao.get("updated"),
            updated_by: dao.get_opt("updated_by"),
            priority: dao.get_opt("priority"),
            name: dao.get_opt("name"),
            description: dao.get_opt("description"),
            help: dao.get_opt("help"),
            active: dao.get("active"),
            category_id: dao.get("category_id"),
        }
    }

    fn to_dao(&self) -> Dao {
        let mut dao = Dao::new();
        match self.organization_id {
            Some(ref _value) => dao.set("organization_id", _value),
            None => dao.set_null("organization_id"),
        }
        match self.client_id {
            Some(ref _value) => dao.set("client_id", _value),
            None => dao.set_null("client_id"),
        }
        dao.set("created", &self.created);
        match self.created_by {
            Some(ref _value) => dao.set("created_by", _value),
            None => dao.set_null("created_by"),
        }
        dao.set("updated", &self.updated);
        match self.updated_by {
            Some(ref _value) => dao.set("updated_by", _value),
            None => dao.set_null("updated_by"),
        }
        match self.priority {
            Some(ref _value) => dao.set("priority", _value),
            None => dao.set_null("priority"),
        }
        match self.name {
            Some(ref _value) => dao.set("name", _value),
            None => dao.set_null("name"),
        }
        match self.description {
            Some(ref _value) => dao.set("description", _value),
            None => dao.set_null("description"),
        }
        match self.help {
            Some(ref _value) => dao.set("help", _value),
            None => dao.set_null("help"),
        }
        dao.set("active", &self.active);
        dao.set("category_id", &self.category_id);
        dao
    }
}

impl IsTable for Category{
    fn table() -> Table {
        Table {
            schema: Some("bazaar".to_string()),
            name: "category".to_string(),
            parent_table: Some("record".to_string()),
            sub_table: vec![],
            comment: None,
            columns: vec![],
            is_view: false,
        }
    }
}

fn main() {
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v8";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();

    let category: Category = Query::insert()
                                 .set("name", &"Test Category12121")
                                 .into_table(&"bazaar.category")
                                 .return_all()
                                 .collect_one(db.as_ref())
                                 .unwrap();
    println!("category: {}", category.name.unwrap());
}
