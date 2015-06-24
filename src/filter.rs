use query::Query;
use dao::Type;

pub enum Connector{
    And,
    Or
}

pub enum Equality{
    EQ, //EQUAL,
    NE, //NOT_EQUAL,
    LT, //LESS_THAN,
    LTE, //LESS_THAN_OR_EQUAL,
    GT, //GREATER_THAN,
    GTE, //GREATER_THAN_OR_EQUAL,
    IN,
    NOTIN,//NOT_IN,
    LIKE,
    NULL,
    NOTNULL,//NOT_NULL,
    ISNULL,//IS_NULL,
}

pub enum Operand{
    Query(Query),
    Value(Type),
}

/// TODO: support for functions on columns
pub struct Filter{
    pub connector:Connector,
    /// TODO: maybe renamed to LHS, supports functions and SQL
    pub column:String,
    pub equality:Equality,
    /// TODO: RHS, supports functions and SQL
    pub operand:Operand,
    pub subfilters:Vec<Filter>
}

impl Filter{

    pub fn new(column:&str, equality:Equality, operand:Operand)->Self{
        Filter{
            connector:Connector::And,
            column:column.to_string(),
            equality:equality,
            operand:operand,
            subfilters:Vec::new(),
        }
    }
    
    pub fn and(mut self, column:&str, equality:Equality, operand:Operand)->Self{
        let mut filter = Filter::new(column, equality, operand);
        filter.connector = Connector::And;
        self.subfilters.push(filter);
        self
    }
    
    pub fn or(mut self, column:&str, equality:Equality, operand:Operand)->Self{
        let mut filter = Filter::new(column, equality, operand);
        filter.connector = Connector::Or;
        self.subfilters.push(filter);
        self
    }
    
}