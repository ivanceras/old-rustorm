use std::collections::HashMap;
use table::Table;
use lookup::TableLookup;

pub struct SampleTableLookup{
	list:HashMap<String, Table>
}

impl TableLookup for SampleTableLookup {
	
	fn get(&self, table:String)->Option<&Table>{
		self.list.get(&table)
	}
	
}