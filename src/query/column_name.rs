use query::HasEquality;
use dao::ToValue;
use query::Filter;
use query::Equality;
use std::fmt;
use table::Column;
use query::Condition;
use query::Operand;
use query::Connector;
use query::operand::ToOperand;

#[derive(Clone)]
#[derive(Debug)]
#[derive(RustcEncodable, RustcDecodable)]
pub struct ColumnName {
    pub column: String,
    pub table: Option<String>,
    // //optional schema, if ever there are same tables resideing in  different schema/namespace
    pub schema: Option<String>,
}

impl ColumnName {
    pub fn from_str(column: &str) -> Self {
        if column.contains(".") {
            let splinters = column.split(".").collect::<Vec<&str>>();
            assert!(splinters.len() == 2, "There should only be 2 splinters");
            let table_split = splinters[0].to_owned();
            let column_split = splinters[1].to_owned();
            ColumnName {
                column: column_split.to_owned(),
                table: Some(table_split.to_owned()),
                schema: None,
            }
        } else {
            ColumnName {
                column: column.to_owned(),
                table: None,
                schema: None,
            }
        }
    }

    pub fn default_rename(&self) -> String {
        match self.table {
            Some(ref s) => format!("{}.{}", s, self.column),
            None => {
                panic!("Unable to rename {} since table is not specified",
                           self.column)
            }
        }
    }

    /// table name and column name
    pub fn complete_name(&self) -> String {
        match self.table {
            Some(ref s) => format!("{}.{}", s, self.column),
            None => self.column.to_owned(),
        }
    }
    /// includes the schema, table name and column name
    pub fn super_complete_name(&self) -> String {
        match self.schema {
            Some(ref s) => format!("{}.{}", s, self.complete_name()),
            None => self.complete_name(),
        }
    }

    /// is this column conflicts the other column
    /// conflicts means, when used both in a SQL query, it will result to ambiguous columns
    pub fn is_conflicted(&self, other: &ColumnName) -> bool {
        self.column == other.column
    }
}


pub trait ToColumnName {
    fn to_column_name(&self) -> ColumnName;
}

impl ToColumnName for Column {
    fn to_column_name(&self) -> ColumnName {
        ColumnName {
            table: self.table.to_owned(),
            column: self.name.to_owned(),
            schema: None,
        }
    }
}

impl<'a> ToColumnName for &'a str {
    fn to_column_name(&self) -> ColumnName {
        ColumnName::from_str(self)
    }
}


impl fmt::Display for ColumnName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.complete_name())
    }
}

impl PartialEq for ColumnName {
    fn eq(&self, other: &Self) -> bool {
        self.column == other.column && self.table == other.table
    }

    fn ne(&self, other: &Self) -> bool {
        self.column != other.column || self.table != other.table || self.schema != other.schema
    }
}
