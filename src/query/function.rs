
use query::Operand;
use query::{ColumnName,ToColumnName};
use query::operand::ToOperand;


/// function in a sql statement
#[derive(Debug)]
#[derive(Clone)]
pub struct Function {
    pub function: String,
    pub params: Vec<Operand>,
}

pub fn COUNT(to_operand: &ToOperand)->Operand{
	Operand::Function(
		Function{
			function: "COUNT".to_owned(),
			params: vec![to_operand.to_operand()]
		}
	)
}
