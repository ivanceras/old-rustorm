use query::Operand;
use dao::ToValue;
use query::{ColumnName,ToColumnName};
use dao::Value;
use std::ops::BitAnd;
use std::ops::BitOr;
use query::operand::ToOperand;

/// expression has left operand,
/// equality and right operand
#[derive(Debug)]
#[derive(Clone)]
pub struct Condition {
    pub left: Operand,
    pub equality: Equality,
    pub right: Operand,
}
///
/// Filter struct merged to query
///
#[derive(Debug)]
#[derive(Clone)]
pub enum Connector {
    And,
    Or,
}

#[derive(Debug)]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub enum Equality {
    EQ, // EQUAL,
    NEQ, // NOT_EQUAL,
    LT, // LESS_THAN,
    LTE, // LESS_THAN_OR_EQUAL,
    GT, // GREATER_THAN,
    GTE, // GREATER_THAN_OR_EQUAL,
    IN,
    NOT_IN, // NOT_IN,
    LIKE,
    ILIKE, //add ILIKE
    IS_NOT_NULL, // NOT_NULL,
    IS_NULL, // IS_NULL,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Filter {
    pub connector: Connector,
    pub condition: Condition,
    pub sub_filters: Vec<Filter>, 
}

impl Filter {

    /// user friendly, commonly use API
    pub fn new(column: &str, equality: Equality, value: &ToValue) -> Self {
        let right = Operand::Value(value.to_db_type());
        Filter {
            connector: Connector::And,
            condition: Condition {
                left: Operand::ColumnName(column.to_column_name()),
                equality: equality,
                right: right,
            },
            sub_filters: vec![],
        }
    }


	pub fn AND(mut self, filter: Filter)->Self{
		let mut filter = filter.clone();
		filter.connector = Connector::And;
		self.sub_filters.push(filter);
		self
	}
	pub fn OR(mut self, filter: Filter)->Self{
		let mut filter = filter.clone();
		filter.connector = Connector::Or;
		self.sub_filters.push(filter);
		self
	}

}



pub trait HasEquality{
	
	fn EQ(&self, to_operand: &ToOperand)->Filter;
	fn NEQ(&self, to_operand: &ToOperand)->Filter;
	fn GT(&self, to_operand: &ToOperand)->Filter;
	fn GTE(&self, to_operand: &ToOperand)->Filter;
	fn LT(&self, to_operand: &ToOperand)->Filter;
	fn LTE(&self, to_operand: &ToOperand)->Filter;
	fn LIKE(&self, to_value: &ToValue)->Filter;
	fn ILIKE(&self, to_value: &ToValue)->Filter;
    fn IS_NULL(&self)->Filter;
    fn IS_NOT_NULL(&self)->Filter;
	fn IN(&self, to_operand: &ToOperand)->Filter;
	fn NOT_IN(&self, to_operand: &ToOperand)->Filter;
}

/// implementation of HasEquality for objects that can yield Operand
impl <T>HasEquality for T where T:ToOperand{
	
	fn EQ(&self, to_operand: &ToOperand)->Filter{
		let cond = Condition{
			left: self.to_operand(), 
			equality: Equality::EQ,
			right: to_operand.to_operand() 
		};
		Filter{
			connector: Connector::And,
			condition:cond,
			sub_filters: vec![]
		}
	}
	fn GT(&self, to_operand: &ToOperand)->Filter{
		let cond = Condition{
			left: self.to_operand(), 
			equality: Equality::GT,
			right: to_operand.to_operand() 
		};
		Filter{
			connector: Connector::And,
			condition:cond,
			sub_filters: vec![]
		}
	}
	fn NEQ(&self, to_operand: &ToOperand)->Filter{
        unimplemented!()
    }
	fn GTE(&self, to_operand: &ToOperand)->Filter{
        unimplemented!()
    }

	fn LT(&self, to_operand: &ToOperand)->Filter{
        unimplemented!()
    }

	fn LTE(&self, to_operand: &ToOperand)->Filter{
        unimplemented!()
    }

	fn LIKE(&self, to_value: &ToValue)->Filter{
        unimplemented!()
    }

	fn ILIKE(&self, to_value: &ToValue)->Filter{
        unimplemented!()
    }

    fn IS_NULL(&self)->Filter{
        unimplemented!()
    }

    fn IS_NOT_NULL(&self)->Filter{
        unimplemented!()
    }

	fn IN(&self, to_operand: &ToOperand)->Filter{
        unimplemented!()
    }

	fn NOT_IN(&self, to_operand: &ToOperand)->Filter{
        unimplemented!()
    }
}

