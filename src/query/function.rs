
use query::Operand;
use query::{ColumnName, ToColumnName};
use query::operand::ToOperand;
use query::source::QuerySource;


/// function in a sql statement
#[derive(Debug)]
#[derive(Clone)]
pub struct Function {
    pub function: String,
    pub params: Vec<Operand>,
}


/// A database function COUNT
pub fn COUNT(to_operand: &ToOperand) -> Operand {
    Operand::QuerySource(QuerySource::Function(Function {
        function: "COUNT".to_owned(),
        params: vec![to_operand.to_operand()],
    }))
}

/// database function MAX
pub fn MAX(to_operand: &ToOperand) -> Operand {
    Operand::QuerySource(QuerySource::Function(Function {
        function: "MAX".to_owned(),
        params: vec![to_operand.to_operand()],
    }))
}
/// database function MIN
pub fn MIN(to_operand: &ToOperand) -> Operand {
    Operand::QuerySource(QuerySource::Function(Function {
        function: "MIN".to_owned(),
        params: vec![to_operand.to_operand()],
    }))
}

/// A database date function which returns the time
/// when the query is executed
pub fn NOW() -> Operand {
    Operand::QuerySource(QuerySource::Function(Function {
        function: "NOW".to_owned(),
        params: vec![],
    }))
}
