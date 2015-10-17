extern crate rustorm;

use rustorm::query::Query;
use rustorm::query::{Filter, Equality};
use rustorm::pool::ManagedPool;
use rustorm::database::Database;
use rustorm::platform::Sqlite;

fn main() {
    let create_sql = r"
CREATE TABLE product_availability (
   --Each product has its own product availability which determines when can it be available for purchase
    product_id uuid NOT NULL , --this is the id of the product
    available boolean,
    always_available boolean,
    stocks numeric DEFAULT 1, --available stock
    available_from timestamp with time zone,
    available_until timestamp with time zone,
    available_day json, open_time time with time zone,
    close_time time with time zone, --closing time
    FOREIGN KEY(product_id) REFERENCES product(product_id)
)
    ";
    Sqlite::extract_comments(create_sql);
}
