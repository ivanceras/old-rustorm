use std::collections::HashMap;
use table::Table;

pub trait TableLookup{
	fn get(&self, table:String)->Option<&Table>;
}