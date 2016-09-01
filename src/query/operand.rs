use query::ColumnName;
use query::TableName;
use query::Function;
use query::Query;
use dao::Value;
use query::Filter;
use table::Column;
use query::HasEquality;
use query::Condition;
use query::Equality;
use query::Connector;
use dao::ToValue;
use query::source::QuerySource;
use query::column_name::ToColumnName;
use std::collections::BTreeMap;
use uuid::Uuid;
use chrono::datetime::DateTime;
use chrono::naive::date::NaiveDate;
use chrono::naive::time::NaiveTime;
use chrono::naive::datetime::NaiveDateTime;
use chrono::offset::utc::UTC;
use rustc_serialize::json::Json;
use chrono::offset::fixed::FixedOffset;

pub trait ToOperand {
    fn to_operand(&self) -> Operand;
}


/// Operands can be columns, values, and query sources such as tables, functions, and other queries
#[derive(Debug)]
#[derive(Clone)]
pub enum Operand {
    ColumnName(ColumnName),
    QuerySource(QuerySource),
    Value(Value),
    Vec(Vec<Operand>),
    None,
}
/// work around for &ToOperand argument for Operand
impl ToOperand for Operand {
    fn to_operand(&self) -> Operand {
        self.to_owned()
    }
}



impl<F> ToOperand for F
    where F: Fn() -> Column
{
    fn to_operand(&self) -> Operand {
        Operand::ColumnName(self().to_column_name())
    }
}

/// implementation to convert Function that returns Column to yield an Operand
macro_rules! impl_to_operand_for_fn_column{
	($x:expr) => {
		impl <F>ToOperand for [F;$x] where F:Fn()->Column{
			fn to_operand(&self)->Operand{
				let mut operands = vec![];
				for c in self{
					let column = c();
					let operand = Operand::ColumnName(column.to_column_name());
					operands.push(operand);
				}
				Operand::Vec(operands)
			}
		}
	}
}


impl_to_operand_for_fn_column!(1);
impl_to_operand_for_fn_column!(2);
impl_to_operand_for_fn_column!(3);
impl_to_operand_for_fn_column!(4);
impl_to_operand_for_fn_column!(5);
impl_to_operand_for_fn_column!(6);
impl_to_operand_for_fn_column!(7);
impl_to_operand_for_fn_column!(8);
impl_to_operand_for_fn_column!(9);
impl_to_operand_for_fn_column!(10);
impl_to_operand_for_fn_column!(11);
impl_to_operand_for_fn_column!(12);



impl ToOperand for [&'static str; 1] {
    fn to_operand(&self) -> Operand {
        Operand::ColumnName(self[0].to_column_name())
    }
}

macro_rules! impl_to_operand_for_static_str_array{
    ($x:expr) => (
		impl ToOperand for [&'static str;$x]{
			fn to_operand(&self)->Operand{
				let mut operands = vec![];
				for s in self{
					let col = s.to_column_name();
					operands.push(Operand::ColumnName(col));
				}	
				Operand::Vec(operands)
			}
		} 
	);
}

impl_to_operand_for_static_str_array!(2);
impl_to_operand_for_static_str_array!(3);
impl_to_operand_for_static_str_array!(4);
impl_to_operand_for_static_str_array!(5);
impl_to_operand_for_static_str_array!(6);
impl_to_operand_for_static_str_array!(7);
impl_to_operand_for_static_str_array!(8);
impl_to_operand_for_static_str_array!(9);
impl_to_operand_for_static_str_array!(10);
impl_to_operand_for_static_str_array!(11);
impl_to_operand_for_static_str_array!(12);

// TODO: Determine why does conflict to impl ToOperand for Fn()->Column
//
// impl <T>ToOperand for T where T:ToValue{
// fn to_operand(&self)->Operand{
// Operand::Value(self.to_db_type())
// }
// }
//




/// static str is threaded as column
// all other types are values

// Note: &'static str is treated as Column
impl ToOperand for &'static str {
    fn to_operand(&self) -> Operand {
        Operand::ColumnName(self.to_column_name())
    }
}

/// String is treated as Value::String
impl ToOperand for String {
    fn to_operand(&self) -> Operand {
        Operand::Value(Value::String(self.to_owned()))
    }
}

impl ToOperand for Json {
    fn to_operand(&self) -> Operand {
        let json_string = format!("{}", self.pretty());
        Operand::Value(self.to_db_type())
    }
}

/// A workaround for the conflicts in ToOperand for <T:ToValue>

macro_rules! impl_to_operand_for_to_value{
	($t:ty, $i:ident) => (
		impl ToOperand for $t{
			fn to_operand(&self)->Operand{
				Operand::Value(Value::$i(self.to_owned()))
			}
		}
		
	);
}

impl_to_operand_for_to_value!(bool, Bool);
impl_to_operand_for_to_value!(i8, I8);
impl_to_operand_for_to_value!(i16, I16);
impl_to_operand_for_to_value!(i32, I32);
impl_to_operand_for_to_value!(i64, I64);
impl_to_operand_for_to_value!(u8, U8);
impl_to_operand_for_to_value!(u16, U16);
impl_to_operand_for_to_value!(u32, U32);
impl_to_operand_for_to_value!(u64, U64);
impl_to_operand_for_to_value!(f32, F32);
impl_to_operand_for_to_value!(f64, F64);
impl_to_operand_for_to_value!(Vec<u8>, VecU8);
impl_to_operand_for_to_value!(Uuid,  Uuid);
impl_to_operand_for_to_value!(DateTime<FixedOffset>, DateTime);
impl_to_operand_for_to_value!(NaiveDate, NaiveDate);
impl_to_operand_for_to_value!(NaiveTime, NaiveTime);
impl_to_operand_for_to_value!(NaiveDateTime, NaiveDateTime);
