use query::TableName;
use table::Table;
use query::Condition;
use query::Filter;
use query::table_name::ToTableName;


#[derive(Debug)]
#[derive(Clone)]
pub enum JoinType {
    CROSS,
    INNER,
    OUTER,
    NATURAL,
}
#[derive(Debug)]
#[derive(Clone)]
pub enum Modifier {
    LEFT,
    RIGHT,
    FULL,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Join {
    pub modifier: Option<Modifier>,
    pub join_type: Option<JoinType>,
    pub table_name: TableName,
    pub on: Filter,
}


pub trait ToJoin {
    fn ON(&self, filter: Filter) -> Join;
}

impl<F> ToJoin for F
    where F: Fn() -> Table
{
    fn ON(&self, filter: Filter) -> Join {
        let table = self();
        Join {
            modifier: None,
            join_type: None,
            table_name: table.to_table_name(),
            on: filter,
        }
    }
}

impl<'a> ToJoin for &'a str {
    fn ON(&self, filter: Filter) -> Join {
        Join {
            modifier: None,
            join_type: None,
            table_name: self.to_table_name(),
            on: filter,
        }
    }
}
