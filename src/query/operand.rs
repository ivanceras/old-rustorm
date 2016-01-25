use query::ColumnName;
use query::TableName;
use query::Function;
use query::Query;
use dao::Value;
use query::Filter;
use query::ToColumnName;
use table::Column;
use query::HasEquality;
use query::Condition;
use query::Equality;
use query::Connector;
use dao::ToValue;

pub trait ToOperand{
	
	fn to_operand(&self)->Operand;
}


/// Operands can be columns, functions, query or value types
#[derive(Debug)]
#[derive(Clone)]
pub enum Operand {
    ColumnName(ColumnName),
    TableName(TableName),
    Function(Function),
    Query(Query),
    Value(Value),
    Vec(Vec<Operand>),
}


impl HasEquality for Operand{
	fn EQ(&self, to_operand: &ToOperand)->Filter{
		let condition = Condition{
			left: self.to_owned(),
			equality: Equality::EQ,
			right: to_operand.to_operand()
		};
		Filter{
			connector: Connector::And,
			condition: condition,
			sub_filters: vec![]
		}
	}

	fn GT(&self, to_operand: &ToOperand)->Filter{
		let condition = Condition{
			left: self.to_owned(),
			equality: Equality::GT,
			right: to_operand.to_operand()
		};
		Filter{
			connector: Connector::And,
			condition: condition,
			sub_filters: vec![]
		}
	}
	fn EQ_VALUE(&self, to_value: &ToValue)->Filter{
		let cond = Condition{
			left: self.to_owned(), 
			equality: Equality::EQ,
			right: Operand::Value(to_value.to_db_type())
		};
		Filter{
			connector: Connector::And,
			condition:cond,
			sub_filters: vec![]
		}
	}
}

impl <F>ToOperand for F where F:Fn()->Column{
	fn to_operand(&self)->Operand{
		let column_name = self();
		Operand::ColumnName(column_name.to_column_name())
	}
} 
impl <F>ToOperand for [F;1] where F:Fn()->Column{
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


impl ToOperand for &'static str{
	fn to_operand(&self)->Operand{
		Operand::ColumnName(self.to_column_name())
	}
} 

impl ToOperand for [&'static str;1]{
	fn to_operand(&self)->Operand{
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

impl ToOperand for (&'static str, &'static str){
	fn to_operand(&self)->Operand{
		let &(a, b) = self;
		let mut operand = vec![];
		operand.push(Operand::ColumnName(a.to_column_name()));
		operand.push(Operand::ColumnName(b.to_column_name()));
		Operand::Vec(operand)
	}
}
impl ToOperand for (&'static str, &'static str, &'static str){
	fn to_operand(&self)->Operand{
		let &(a, b, c) = self;
		let mut operand = vec![];
		operand.push(Operand::ColumnName(a.to_column_name()));
		operand.push(Operand::ColumnName(b.to_column_name()));
		operand.push(Operand::ColumnName(c.to_column_name()));
		Operand::Vec(operand)
	}
}

impl ToOperand for i32{
	fn to_operand(&self)->Operand{
		Operand::Value(Value::I32(*self))
	}
}

