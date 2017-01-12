use query::Operand;
use query::ColumnName;
use query::ToTableName;
use query::operand::ToOperand;
use query::source::{SourceField, QuerySource, ToSourceField};
use query::table_name::TableName;



#[derive(Debug)]
#[derive(Clone)]
pub struct Field {
    /// the field
    pub operand: Operand,
    /// when renamed as field
    pub name: Option<String>,
}

impl Field {
    pub fn rename(&self) -> Field {
        match self.operand {
            Operand::ColumnName(ref column_name) => {
                let rename = column_name.default_rename();
                Field {
                    operand: Operand::ColumnName(column_name.clone()),
                    name: Some(rename),
                }
            }
            // renames should only be called on column names
            _ => unreachable!(),
        }
    }
}

pub trait ToField {
    fn to_field(&self) -> Vec<Field>;
}



pub trait Rename {
    fn AS(&self, s: &str) -> SourceField;
}

impl Rename for &'static str {
    fn AS(&self, s: &str) -> SourceField {
        let table_name = TableName::from_str(self);
        let qs = QuerySource::TableName(table_name);
        SourceField {
            source: qs,
            rename: Some(s.to_owned()),
        }
    }
}

impl ToSourceField for SourceField {
    fn to_source_field(&self) -> Vec<SourceField> {
        vec![self.to_owned()]
    }
}

// TODO use Iterator<Item=T> for implementing all other loopable types
macro_rules! impl_to_source_field_for_source_field{
	($x:expr) => (
		impl ToSourceField for [SourceField;$x]{
			fn to_source_field(&self)->Vec<SourceField>{
				let mut sources = vec![];
				for s in self{
					sources.push(s.to_owned());
				}
				sources
			}
		}
	);
}

impl_to_source_field_for_source_field!(1);
impl_to_source_field_for_source_field!(2);
impl_to_source_field_for_source_field!(3);
impl_to_source_field_for_source_field!(4);
impl_to_source_field_for_source_field!(5);
impl_to_source_field_for_source_field!(6);
impl_to_source_field_for_source_field!(7);
impl_to_source_field_for_source_field!(8);
impl_to_source_field_for_source_field!(9);
impl_to_source_field_for_source_field!(10);
impl_to_source_field_for_source_field!(11);
impl_to_source_field_for_source_field!(12);


impl ToField for Field {
    fn to_field(&self) -> Vec<Field> {
        vec![self.to_owned()]
    }
}

macro_rules! impl_to_field_for_field{
	($x:expr) => (
		impl ToField for [Field;$x]{
			fn to_field(&self)->Vec<Field>{
				let mut fields = vec![];
				for s in self{
					fields.push(s.to_owned())
				}
				fields
			}
		}
	);
}

impl<T> ToField for T
    where T: ToOperand
{
    fn to_field(&self) -> Vec<Field> {
        let operand = self.to_operand();
        vec![Field {
                 operand: operand,
                 name: None,
             }]
    }
}

impl_to_field_for_field!(1);
impl_to_field_for_field!(2);
impl_to_field_for_field!(3);
impl_to_field_for_field!(4);
impl_to_field_for_field!(5);
impl_to_field_for_field!(6);
impl_to_field_for_field!(7);
impl_to_field_for_field!(8);
impl_to_field_for_field!(9);
impl_to_field_for_field!(10);
impl_to_field_for_field!(11);
impl_to_field_for_field!(12);
