use query::ColumnName;
use std::fmt;
use table::Table;
use query::Join;
use query::Filter;


#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct TableName {
    pub schema: Option<String>,
    pub name: String,
    /// optional columns needed when rename for conflicting columns are needed
    pub columns: Vec<ColumnName>,
}

impl TableName {

    pub fn from_str(str: &str) -> Self {
        if str.contains(".") {
            let splinters = str.split(".").collect::<Vec<&str>>();
            assert!(splinters.len() == 2, "There should only be 2 splinters");
            let schema_split = splinters[0].to_owned();
            let table_split = splinters[1].to_owned();

            TableName {
                schema: Some(schema_split),
                name: table_split,
                columns: vec![],
            }

        } else {
            TableName {
                schema: None,
                name: str.to_owned(),
                columns: vec![],
            }
        }
    }

    pub fn complete_name(&self) -> String {
        match self.schema {
            Some (ref schema) => format!("{}.{}", schema, self.name),
            None => self.name.to_owned(),
        }
    }
}

impl PartialEq for TableName {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.schema == other.schema
    }

    fn ne(&self, other: &Self) -> bool {
        self.name != other.name || self.schema != other.schema
    }
}

impl fmt::Display for TableName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.complete_name())
    }
}

/// convert str, IsTable to TableName
pub trait ToTableName {

    fn to_table_name(&self) -> TableName;

}

impl <'a>ToTableName for &'a str {

    fn to_table_name(&self) -> TableName {
        TableName::from_str(self)
    }
}

impl <F>ToTableName for F where F: Fn() -> Table{
	fn to_table_name(&self) -> TableName{
		let table = self();
		debug!("table: {:?}", table);
		table.to_table_name()
	}
}

impl ToTableName for Table {

    /// contain the columns for later use when renaming is necessary
    fn to_table_name(&self) -> TableName {
        let mut columns = vec![];
        for c in &self.columns {
            let column_name = ColumnName {
                schema: self.schema.clone(),
                table: Some(self.name.to_owned()),
                column: c.name.to_owned(),
            };
            columns.push(column_name);
        }
        TableName {
            schema: self.schema.clone(),
            name: self.name.to_owned(),
            columns: columns,
        }
    }
}
