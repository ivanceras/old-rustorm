use query::Operand;
use query::Field;
use query::TableName;
use query::Query;
use query::Function;
use query::column_name::{ColumnName, ToColumnName};
use table::Column;
use table::Table;
use query::table_name::ToTableName;
use query::operand::ToOperand;


/// This fields can be used in the FROM field
#[derive(Debug)]
#[derive(Clone)]
pub enum QuerySource {
    TableName(TableName),
    Query(Query),
    Function(Function),
}

/// QuerySource fields can be renamed
#[derive(Debug)]
#[derive(Clone)]
pub struct SourceField {
    pub source: QuerySource,
    pub rename: Option<String>,
}


pub trait ToSourceField {
    fn to_source_field(&self) -> Vec<SourceField>;
}


impl ToSourceField for &'static str {
    fn to_source_field(&self) -> Vec<SourceField> {
        let table_name = TableName::from_str(self);
        let qs = QuerySource::TableName(table_name);
        vec![SourceField{
			source: qs,
			rename: None,
		}]
    }
}

impl ToSourceField for String {
    fn to_source_field(&self) -> Vec<SourceField> {
        let table_name = TableName::from_str(self);
        let qs = QuerySource::TableName(table_name);
        vec![SourceField{
			source: qs,
			rename: None,
		}]
    }
}
impl ToSourceField for Table {
    fn to_source_field(&self) -> Vec<SourceField> {
        let table_name = self.to_table_name();
        let qs = QuerySource::TableName(table_name);
        vec![SourceField{
			source: qs,
			rename: None,
		}]
    }
}

impl ToSourceField for QuerySource {
    fn to_source_field(&self) -> Vec<SourceField> {
        vec![
			SourceField{
				source: self.to_owned(),
				rename: None
			}
		]
    }
}

macro_rules! impl_to_source_field_for_static_str{
	($x:expr) => (
		impl ToSourceField for [&'static str;$x]{
			fn to_source_field(&self)->Vec<SourceField>{
				let mut sources = vec![];
				for s in self{
					let table_name = TableName::from_str(s);
					let qs = QuerySource::TableName(table_name);
					let sf = SourceField{
						source: qs,
						rename: None
					};
					sources.push(sf);
				}
				sources
			}
		}
	);
}

impl_to_source_field_for_static_str!(1);
impl_to_source_field_for_static_str!(2);
impl_to_source_field_for_static_str!(3);

impl<F> ToSourceField for F
    where F: Fn() -> Table
{
    fn to_source_field(&self) -> Vec<SourceField> {
        let table_name = self().to_table_name();
        let qs = QuerySource::TableName(table_name);
        let sf = SourceField {
            source: qs,
            rename: None,
        };
        vec![sf]
    }
}
