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


    pub fn is_null(column: &str) -> Self {
        Filter::new(column, Equality::IS_NULL, &())
    }
    pub fn is_not_null(column: &str) -> Self {
        Filter::new(column, Equality::IS_NOT_NULL, &())
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
	
	fn EQ(&self, to_column: &ToOperand)->Filter;
	fn GT(&self, operand: &ToOperand)->Filter;
	fn EQ_VALUE(&self, to_column: &ToValue)->Filter;
}

impl <'a> HasEquality for &'a str{
	
	fn EQ(&self, to_operand: &ToOperand)->Filter{
		let col = self.to_column_name();
		let cond = Condition{
			left: Operand::ColumnName(col), 
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
		let col = self.to_column_name();
		let cond = Condition{
			left: Operand::ColumnName(col), 
			equality: Equality::GT,
			right: to_operand.to_operand() 
		};
		Filter{
			connector: Connector::And,
			condition:cond,
			sub_filters: vec![]
		}
	}
	fn EQ_VALUE(&self, to_value: &ToValue)->Filter{
		let col = self.to_column_name();
		let cond = Condition{
			left: Operand::ColumnName(col), 
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
