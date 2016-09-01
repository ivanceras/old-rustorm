use query::Operand;
use dao::ToValue;
use query::{ColumnName, ToColumnName};
use dao::Value;
use std::ops::BitAnd;
use std::ops::BitOr;
use query::operand::ToOperand;
use dao::Type;

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
    ILIKE, // add ILIKE
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


    pub fn AND(mut self, filter: Filter) -> Self {
        let mut filter = filter.clone();
        filter.connector = Connector::And;
        self.sub_filters.push(filter);
        self
    }
    pub fn OR(mut self, filter: Filter) -> Self {
        let mut filter = filter.clone();
        filter.connector = Connector::Or;
        self.sub_filters.push(filter);
        self
    }
}



pub trait HasEquality {
    fn EQ(&self, to_operand: &ToOperand) -> Filter;
    fn NEQ(&self, to_operand: &ToOperand) -> Filter;
    fn GT(&self, to_operand: &ToOperand) -> Filter;
    fn GTE(&self, to_operand: &ToOperand) -> Filter;
    fn LT(&self, to_operand: &ToOperand) -> Filter;
    fn LTE(&self, to_operand: &ToOperand) -> Filter;
    fn LIKE(&self, to_value: &ToValue) -> Filter;
    fn ILIKE(&self, to_value: &ToValue) -> Filter;
    fn IS_NULL(&self) -> Filter;
    fn IS_NOT_NULL(&self) -> Filter;
    fn IN(&self, to_operand: &ToOperand) -> Filter;
    fn NOT_IN(&self, to_operand: &ToOperand) -> Filter;
}

macro_rules! fn_has_equality_operand{
    ($f:ident, $eq:expr) => (
        fn $f(&self, to_operand: &ToOperand)->Filter{
            let cond = Condition{
                left: self.to_operand(), 
                equality: $eq,
                right: to_operand.to_operand() 
            };
            Filter{
                connector: Connector::And,
                condition:cond,
                sub_filters: vec![]
            }
        }
    )
}

macro_rules! fn_has_equality_to_value{
    ($f:ident, $eq:expr) => (
        fn $f(&self, to_value: &ToValue)->Filter{
			let cond = Condition{
				left: self.to_operand(), 
				equality: $eq,
				right: Operand::Value(to_value.to_db_type()) 
			};
			Filter{
				connector: Connector::And,
				condition:cond,
				sub_filters: vec![]
			}
		}
    )
}

macro_rules! fn_has_equality_nulls{
	($f:ident, $eq: expr) => (
		fn $f(&self)->Filter{
			let cond = Condition{
				left: self.to_operand(), 
				equality: $eq,
				right: Operand::None,
			};
			Filter{
				connector: Connector::And,
				condition:cond,
				sub_filters: vec![]
			}
		}
	)
}

/// implementation of HasEquality for objects that can yield Operand
impl<T> HasEquality for T
    where T: ToOperand
{
    fn_has_equality_operand!(EQ, Equality::EQ);
    fn_has_equality_operand!(NEQ, Equality::NEQ);
    fn_has_equality_operand!(GT, Equality::GT);
    fn_has_equality_operand!(GTE, Equality::GTE);
    fn_has_equality_operand!(LT, Equality::LT);
    fn_has_equality_operand!(LTE, Equality::LTE);
    fn_has_equality_operand!(IN, Equality::IN);
    fn_has_equality_operand!(NOT_IN, Equality::NOT_IN);
    fn_has_equality_to_value!(LIKE, Equality::LIKE);
    fn_has_equality_to_value!(ILIKE, Equality::ILIKE);
    fn_has_equality_nulls!(IS_NULL, Equality::IS_NULL);
    fn_has_equality_nulls!(IS_NOT_NULL, Equality::IS_NOT_NULL);
}
