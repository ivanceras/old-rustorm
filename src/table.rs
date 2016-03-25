use std::fmt;
use dao::Type;
use query::Operand;

#[derive(Debug, Clone, PartialEq)]
pub struct Foreign {
    pub schema: Option<String>,
    pub table: String,
    pub column: String, 
}

impl Foreign{
	
	pub fn from_str(schema_table: &str, column: &str) -> Self{
		if schema_table.contains("."){
			let splinters = schema_table.split(".").collect::<Vec<&str>>();
			assert!(splinters.len() == 2, "There should only be 2 parts");
			let schema = splinters[0].to_owned();
			let table = splinters[1].to_owned();
			Foreign{
				schema: Some(schema),
				table: table,
				column: column.to_owned(),
			}
		}else{
			Foreign{
				schema: None,
				table: schema_table.to_owned(),
				column: column.to_owned(),
			}
		}
	}

    pub fn complete_table_name(&self) -> String {
        match self.schema {
            Some (ref schema) => format!("{}.{}", schema, self.table),
            None => self.table.to_owned(),
        }
    }

}

#[derive(Debug, Clone)]
pub struct Column {
	pub table: Option<String>,
    pub name: String,
    /// the generic data type, ie: u32, f64, string
    pub data_type: Type,
    /// the database data type of this column, ie: int, numeric, character varying
    pub db_data_type: String,
    pub is_primary: bool,
    pub is_unique: bool,
    pub default: Option<Operand>,
    pub comment: Option<String>,
    pub not_null: bool,
    pub foreign: Option<Foreign>,
    ///determines if the column is inherited from the parent table
    pub is_inherited: bool,
}

impl Column {

    fn is_keyword(str: &str) -> bool {
        let keyword = ["type", "yield", "macro"];
        keyword.contains(&str)
    }
    
    pub fn nullable(&self) -> bool{
        !self.not_null
    }


    ///some column names may be a rust reserve keyword, so have to correct them
    pub fn corrected_name(&self) -> String {
        if Self::is_keyword(&self.name) {
            warn!("Warning: {} is rust reserved keyword", self.name);
            return format!("{}_", self.name);
        }
        self.name.to_owned()
    }

    pub fn displayname(&self) -> String {
        let clean_name = self.clean_name();
        clean_name.replace("_", " ")
    }

    /// presentable display names, such as removing the ids if it ends with one
    fn clean_name(&self) -> String {
        if self.name.ends_with("_id") {
            return self.name.trim_right_matches("_id").to_owned();
        }
        self.name.to_owned()
    }

    /// shorten, compress the name based on the table it points to
    /// parent_organization_id becomes parent
    pub fn condense_name(&self) -> String {
        let clean_name = self.clean_name();
        if let Some(ref foreign) = self.foreign {
            let foreign = foreign.clone();
            if clean_name.len() > foreign.table.len() {
                return clean_name.trim_right_matches(&foreign.table)
                    .trim_right_matches("_")
                    .to_owned();
            }
        }
        clean_name
    }

}


impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialEq for Column {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }

    fn ne(&self, other: &Self) -> bool {
        self.name != other.name
    }
}

/// trait for table definition
pub trait IsTable {
    fn table() -> Table;
}

/// all referenced table used in context
#[allow(dead_code)]
pub struct RefTable<'a> {
    /// the table being referred
    pub table: &'a Table,
    /// the referring column, applicable to direct has_one
    column: Option<&'a Column>,
    linker_table: Option<&'a Table>,
    pub is_ext: bool,
    pub is_has_one: bool,
    pub is_has_many: bool,
    pub is_direct: bool,
}

/// FIXME need more terse and ergonomic handling of conflicting member names
impl <'a>RefTable<'a> {

    /// return the appropriate member name of this reference
    /// when used with the table in context
    /// will have to use another name if the comed up name
    /// already in the column names
    /// 1. the concise name of the referred/referrring table
    /// 2. the name of the referred/referring table
    /// 3. the appended column_name and the table name
    /// 4. the table_name appended with HasMany, or HasOne
    /// 1:1, 1:M, M:M
    /// 11, 1m mm
    pub fn member_name(&self, used_in_table: &Table) -> String {
        let has_conflict = false;
        if self.is_has_one {
            if has_conflict {
                let suffix = "_1";
                return format!("{}{}", self.column.unwrap().name, suffix);
            } else {
                return self.column.unwrap().condense_name();
            }
        }
        if self.is_ext {
            if has_conflict {
                let suffix = "_1";
                return format!("{}{}", self.table.name, suffix);
            } else {
                return self.table.condensed_member_name(used_in_table);
            }
        }
        if self.is_has_many && self.is_direct {
            if has_conflict {
                let suffix = "_1m";
                return format!("{}{}", self.table.name, suffix);
            } else {
                return self.table.name.to_owned();
            }

        }
        if self.is_has_many && !self.is_direct {
            if has_conflict {
                let suffix = "_mm";
                return format!("{}{}", self.table.name, suffix);
            } else {
                return self.table.name.to_owned();
            }
        }
        unreachable!();
    }
}


#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
pub struct Table {
    ///which schema this belongs
    pub schema: Option<String>,

    ///the table name
    pub name: String,

    ///the parent table of this table when inheriting (>= postgresql 9.3)
    /// [FIXME] need to tell which schema this parent table belongs
    /// there might be same table in different schemas
    pub parent_table: Option<String>,

    ///what are the other table that inherits this
    /// [FIXME] need to tell which schema this parent table belongs
    /// there might be same table in different schemas
    pub sub_table: Vec<String>,

    ///comment of this table
    pub comment: Option<String>,

    ///columns of this table
    pub columns: Vec<Column>,

    /// views can also be generated
    pub is_view: bool,
}
impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialEq for Table {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.schema == other.schema
    }

    fn ne(&self, other: &Self) -> bool {
        self.name != other.name || self.schema != other.schema
    }
}


impl Table {

	/// create table with name	
	pub fn with_name(schema_table: &str)->Self{
		if schema_table.contains("."){
			let splinters = schema_table.split(".").collect::<Vec<&str>>();
			assert!(splinters.len() == 2, "There should only be 2 parts");
			let schema = splinters[0].to_owned();
			let table = splinters[1].to_owned();
			Table{
				schema: Some(schema),
				name: table,
				..Default::default()
			}
		}else{
			Table{
				schema: None,
				name: schema_table.to_owned(),
				..Default::default()
			}
		}
	}

    /// return the long name of the table using schema.table_name
    pub fn complete_name(&self) -> String {
        match self.schema {
            Some (ref schema) => format!("{}.{}", schema, self.name),
            None => self.name.to_owned(),
        }
    }
    /// capitalize the first later, if there is underscore remove it then capitalize the next letter
    pub fn struct_name(&self) -> String {
        let mut struct_name = String::new();
        for i in self.name.split('_') {
            struct_name.push_str(&capitalize(i));
        }
        struct_name
    }

    /// get the display name of this table
    /// product_availability -> Product Availability
    pub fn displayname(&self) -> String {
        let mut display_name = String::new();
        for i in self.name.split('_') {
            display_name.push_str(&capitalize(i));
            display_name.push_str(" ");
        }
        display_name.trim().to_owned()
    }

    /// get a shorter display name of a certain table
    /// when being refered to this table
    /// example product.product_availability -> Availability
    /// user.user_info -> Info
    pub fn condensed_displayname(&self, table: &Table) -> String {
        if self.name.len() > table.name.len() {
            let mut concise_name = String::new();
            for i in self.name.split('_') {
                if table.name != i {
                    concise_name.push_str(&capitalize(i));
                    concise_name.push_str(" ");
                }
            }
            concise_name.trim().to_owned()
        } else {
            self.displayname()
        }
    }

    /// remove plural names such as users to user
    fn clean_name(&self) -> String {
        if self.name.ends_with("s") {
            self.name.trim_right_matches("s").to_owned()

        } else if self.name.ends_with("ies") {
            self.name.trim_right_matches("y").to_owned()

        } else {
            self.name.to_owned()
        }
    }

    /// get a condensed name of this table when used in contex with another table
    pub fn condensed_member_name(&self, used_in_table: &Table) -> String {
        if self.name.len() > used_in_table.name.len() {
            let mut concise_name = String::new();
            let used_in_tablename = used_in_table.clean_name();
            for i in self.name.split('_') {
                if used_in_tablename != i {
                    concise_name.push_str(i);
                    concise_name.push_str("_");
                }
            }
            concise_name.trim_right_matches("_").to_owned()
        } else {
            self.name.to_owned()
        }
    }

    /// determine if this table has a colum named
    pub fn has_column_name(&self, column: &str) -> bool {
        for c in &self.columns {
            if c.name == column {
                return true;
            }
        }
        false
    }

    /// return the column of this table with the name
    pub fn get_column(&self, column: &str) -> Option<Column> {
        let column_name = column.to_owned();
        for c in &self.columns {
            if c.name == column_name {
                return Some(c.clone());
            }
        }
        None
    }

    /// return all the primary columns of this table
    pub fn primary_columns(&self) -> Vec<&Column> {
        let mut primary_columns = Vec::new();
        for c in &self.columns {
            if c.is_primary {
                primary_columns.push(c);
            }
        }
        primary_columns.sort_by(|a, b| a.name.cmp(&b.name));
        primary_columns
    }

    pub fn non_nullable_columns(&self) -> Vec<String> {
        let mut non_nulls = vec![];
        for c in &self.columns {
            if c.not_null {
                non_nulls.push(c.name.to_owned());
            }
        }
        non_nulls
    }

    /// return all the columns of this table excluding the inherited columns
    pub fn uninherited_columns(&self) -> Vec<&Column> {
        let mut included = Vec::new();
        let mut uninherited_columns = Vec::new();
        for c in &self.columns {
            if !c.is_inherited && !included.contains(&&c.name) {
                uninherited_columns.push(c);
                included.push(&c.name);
            }
        }
        uninherited_columns.sort_by(|a, b| a.name.cmp(&b.name));
        uninherited_columns
    }

    /// return all the inherited columns
    pub fn inherited_columns(&self) -> Vec<&Column> {
        let mut included = Vec::new();
        let mut inherited_columns = Vec::new();
        for c in &self.columns {
            if c.is_inherited && !included.contains(&&c.name) {
                inherited_columns.push(c);
                included.push(&c.name);
            }
        }
        inherited_columns.sort_by(|a, b| a.name.cmp(&b.name));
        inherited_columns
    }

    /// check to see if the column is a primary or not
    /// the Column.is_primary property is not reliable since it also list down the foreign key
    /// which makes it 2 entries in the table
    pub fn is_primary(&self, column_name: &str) -> bool {
        for p in self.primary_columns() {
            if p.name == column_name {
                return true;
            }
        }
        false
    }
	/// return true when all columns are primary columns
	/// false if at least 1 is not a primary column
    pub fn are_primary_columns(&self, column_names: &Vec<String>) -> bool {
        for c in column_names {
        	if !self.is_primary(&c){
				return false;
			}
		}
       	true 
    }
    /// return all the unique keys of this table
    pub fn unique_columns(&self) -> Vec<&Column> {
        let mut unique_columns = Vec::new();
        for c in &self.columns {
            if c.is_unique {
                unique_columns.push(c);
            }
        }
        unique_columns.sort_by(|a, b| a.name.cmp(&b.name));
        unique_columns
    }

    pub fn foreign_columns(&self) -> Vec<&Column> {
        let mut columns = Vec::new();
        for c in &self.columns {
            if c.foreign.is_some() {
                columns.push(c);
            }
        }
        columns.sort_by(|a, b| a.name.cmp(&b.name));
        columns
    }
	
	fn get_parent_table<'a>(&self, tables: &'a [Table]) -> Option<&'a Table>{
		match &self.parent_table{
			&Some(ref p_table) => {
				let tmp_table = Table::with_name(&p_table);
				Some(Self::get_table(&tmp_table.schema, &tmp_table.name, tables))
			},
			&None => None
		}
	}

	/// tell whether this column exist on the parent column as well.
	/// does the calculation through the structure, may not correctly reflect the database
	fn is_inherited_column(self, column: &str, tables: &[Table])->bool{
		match self.get_parent_table(tables){
			Some(parent_table) =>{
				for column in &self.columns{
					if parent_table.has_column_name(&column.name){
						return true;
					}
				}
				false
			},
			None => false,
		}
	}

	fn same_schema(&self, table: &Table)->bool{
		match &self.schema{
			&None => match &table.schema{
				&None => true,
				&Some(_) => false,
			},
			&Some(ref schema) => match &table.schema{
				&None => false,
				&Some(ref tschema) => (schema == tschema)
			}
		}
	}

    /// return the first match of table name regardless of which schema it belongs to.
    /// get the table definition using the table name from an array of table object
    /// [FIXME] Needs to have a more elegant solution by using HashMap
    pub fn get_table<'a>(schema: &Option<String>, table_name: &str, tables: &'a [Table]) -> &'a Table {
        for t in tables {
            if t.name == table_name && 
				match schema{
					&Some(ref schema) => match &t.schema{
						&Some(ref tschema) => (schema == tschema), 
						&None => false
					},	
					&None => match t.schema{
						Some(_) => false,
						None => true
					} 
				} 
		    {
                return t;
            }
        }
        panic!("Table {} is not on the list can not be found", table_name);
    }



    /// get all the tables that is referred by this table
    /// get has_one
    pub fn referred_tables<'a>(&'a self, tables: &'a [Table]) -> Vec<(&'a Column, &'a Table)> {
        let mut referred_tables = Vec::new();
        for c in &self.columns {
            if let Some(ref foreign) = c.foreign {
                let ft = foreign;
                let ftable = Self::get_table(&ft.schema, &ft.table, tables);
                referred_tables.push((c, ftable));
            }
        }
        referred_tables
    }

    /// has_many_direct
    /// get all other tables that is refering to this table
    /// when any column of a table refers to this table
    /// get_has_many
    pub fn referring_tables<'a>(&self, tables: &'a [Table]) -> Vec<(&'a Table, &'a Column)> {
        let mut referring = Vec::new();
        for t in tables {
            for c in &t.columns {
                if let Some(ref foreign) = c.foreign {
                    if self.name == foreign.table {
                        referring.push((t, c));
                    }
                }
            }
        }
        referring
    }


    /// all the referenced table of this table, this is used in building the structs as stubs or final model definitions
    /// it does not include the parent is this table is just an extension to it
    /// when a linker table, no applicable referenced is returned
    /// parent of extension tables are not returned
    pub fn get_all_applicable_reference<'a>(&'a self, all_tables: &'a [Table]) -> Vec<RefTable> {
        let mut applicable_ref = vec![];
        if self.is_linker_table() {
            //println!("Skipping reference listing for table {}, Linker table should not contain objects", self);
            return vec![];
        }
        let all_ref = self.get_all_referenced_table(all_tables);
        for ref_table in all_ref {
            if self.is_extension_of(ref_table.table, all_tables) {
                //println!("skipping master table {} since {} is just an extension to it ",ref_table.table, self);
            } else {
                applicable_ref.push(ref_table)
            }
        }
        applicable_ref
    }

    fn get_all_referenced_table<'a>(&'a self, all_tables: &'a [Table]) -> Vec<RefTable> {
        let mut referenced_tables = vec![];

        let has_one = self.referred_tables(all_tables);
        for (column, table) in has_one {
            let ref_table = RefTable {
                table: table,
                column: Some(column),
                linker_table: None,
                is_has_one: true,
                is_ext: false,
                is_has_many: false,
                is_direct: true,
            };
            referenced_tables.push(ref_table);
        }


        let extension_tables = self.extension_tables(all_tables);
        for ext in &extension_tables {
            let ref_table = RefTable {
                table: ext,
                column: None,
                linker_table: None,
                is_has_one: false,
                is_ext: true,
                is_has_many: false,
                is_direct: true,
            };
            referenced_tables.push(ref_table);
        }

        let has_many_direct = self.referring_tables(all_tables);
        let mut included_has_many = vec![];
        for (hd,column) in has_many_direct {
            if !hd.is_linker_table() && !extension_tables.contains(&hd) &&
                !included_has_many.contains(&hd) {
                    let ref_table = RefTable {
                        table: hd,
                        column: Some(column),
                        linker_table: None,
                        is_has_one: false,
                        is_ext: false,
                        is_has_many: true,
                        is_direct: true,
                    };
                    referenced_tables.push(ref_table);
                    included_has_many.push(hd);
                }
        }
        let has_many_indirect = self.indirect_referring_tables(all_tables);

        for (hi, linker) in has_many_indirect {
            if !hi.is_linker_table() && !extension_tables.contains(&hi) &&
                !included_has_many.contains(&hi) {
                    let ref_table = RefTable {
                        table: hi,
                        column: None,
                        linker_table: Some(linker),
                        is_has_one: false,
                        is_ext: false,
                        is_has_many: true,
                        is_direct: false,
                    };

                    referenced_tables.push(ref_table);
                    included_has_many.push(hi);
                }
        }
        referenced_tables
    }

    ///determine if this table is a linker table
    /// FIXME: make sure that there are 2 different tables referred to it
    pub fn is_linker_table(&self) -> bool {
        let pk = self.primary_columns();
        let fk = self.foreign_columns();
        let uc = self.uninherited_columns();
        if pk.len() == 2 && fk.len() == 2 && uc.len() == 2 {
            return true;
        }
        false
    }

    /// determines if the table is owned by some other table
    /// say order_line is owned by orders
    /// which doesn't make sense to be a stand alone window on its own
    /// characteristic: if it has only 1 has_one which is its owning parent table
    /// and no other direct or indirect referring table
    pub fn is_owned(&self, tables: &[Table]) -> bool {
        let has_one = self.referred_tables(tables);
        let has_many = self.referring_tables(tables);
        has_one.len() == 1 && has_many.is_empty()
    }

    /// has many indirect
    /// when there is a linker table, bypass the 1:1 relation to the linker table
    /// then create a 1:M relation to the other linked table
    /// Algorithmn: determine whether a table is a linker then get the other linked table
    ///        *get all the referring table
    ///        *for each table that refer to this table
    ///        *if there are only 2 columns and is both primary
    ///            and foreign key at the same time
    ///         and 1 of which refer to the primary column of this table
    ///     * then the other table that is refered is the indirect referring table
    /// returns the table that is indirectly referring to this table and its linker table
    pub fn indirect_referring_tables<'a>(&self,
                                         tables: &'a [Table])
                                         -> Vec<(&'a Table, &'a Table)> {
        let mut indirect_referring_tables = Vec::new();
        for (rt, _column) in self.referring_tables(tables) {
            let rt_pk = rt.primary_columns();
            let rt_fk = rt.foreign_columns();
            let rt_uc = rt.uninherited_columns();
            if rt_pk.len() == 2 && rt_fk.len() == 2 && rt_uc.len() == 2 {
                //println!("{} is a candidate linker table for {}", rt.name, self.name);
                let ref_tables = rt.referred_tables(tables);
                let (_, t0) = ref_tables[0];
                let (_, t1) = ref_tables[1];
                let other_table;
                //if self.name == t0.name && self.schema == t0.schema{
                if self == t0 {
                    other_table = t1;
                } else {
                    other_table = t0;
                }
                let mut cnt = 0;
                for fk in &rt_fk {
                    if self.is_foreign_column_refer_to_primary_of_this_table(fk) {
                        cnt += 1;
                    }
                    if other_table.is_foreign_column_refer_to_primary_of_this_table(fk) {
                        cnt += 1;
                    }
                }

                if cnt == 2 {
                    indirect_referring_tables.push((other_table, rt))
                }
            }
        }
        indirect_referring_tables
    }



    /// get referring tables, and check if primary columns of these referring table
    /// is the same set of the primary columns of this table
    /// it is just an extension table
    /// [FIXED]~~FIXME:~~ 2 primary 1 foreign should not be included as extension table
    /// case for photo_sizes
    pub fn extension_tables<'a>(&self, tables: &'a [Table]) -> Vec<&'a Table> {
        let mut extension_tables = Vec::new();
        for (rt, _) in self.referring_tables(tables) {
            let pkfk = rt.primary_and_foreign_columns();
            let rt_pk = rt.primary_columns();
            //if the referring tables's foreign columns are also its primary columns
            //that refer to the primary columns of this table
            //then that table is just an extension table of this table
            if rt_pk == pkfk && !pkfk.is_empty() &&
                //if all fk refer to the primary of this table
                self.are_these_foreign_column_refer_to_primary_of_this_table(&pkfk) {
                    extension_tables.push(rt);
                }
        }
        extension_tables
    }

    /// determines if this table is just an extension of the table specified
    /// extension tables need not to contain a reference of their parent table
    pub fn is_extension_of(&self, table: &Table, all_tables: &[Table]) -> bool {
        let ext_tables = table.extension_tables(all_tables);
        ext_tables.contains(&self)
    }

    /// returns only columns that are both primary and foreign
    /// FIXME: don't have to do this if the function getmeta data has merged this.
    fn primary_and_foreign_columns(&self) -> Vec<&Column> {
        let mut both = Vec::new();
        let pk = self.primary_columns();
        let fk = self.foreign_columns();
        for f in fk {
            if pk.contains(&f) {
                //println!("{}.{} is both primary and foreign", self.name, f.name);
                both.push(f);
            }
        }
        both
    }

    fn is_foreign_column_refer_to_primary_of_this_table(&self, fk: &Column) -> bool {
        if let Some(ref foreign) = fk.foreign {
            let foreign = foreign.clone();
            let table = foreign.table;
            let schema = foreign.schema;
            let column = foreign.column;
            if self.name == table && self.is_primary(&column) &&
				match schema{
					Some(ref schema) => match &self.schema{
						&Some(ref tschema) => (schema == tschema), 
						&None => false
					},	
					None => match &self.schema{
						&Some(_) => false,
						&None => true
					} 
				} 
			{
                return true;
            }
        }
        false
    }

    /// returns the columns of these table that is a foreign columns to the foreign table
    pub fn get_foreign_columns_to_table(&self, foreign_table: &Table) -> Vec<&Column> {
        let mut qualified = vec![];
        let foreign_columns = self.foreign_columns();
        for fc in foreign_columns {
            if foreign_table.is_foreign_column_refer_to_primary_of_this_table(fc) {
                qualified.push(fc)
            }
        }
        qualified
    }

    fn are_these_foreign_column_refer_to_primary_of_this_table(&self,
                                                               rt_fk: &[&Column])
                                                               -> bool {
        let mut cnt = 0;
        for fk in rt_fk {
            if self.is_foreign_column_refer_to_primary_of_this_table(fk) {
                cnt += 1;
            }
        }
        cnt == rt_fk.len()
    }

}


fn capitalize(str: &str) -> String {
    str.chars()
        .take(1)
        .flat_map(char::to_uppercase)
        .chain(str.chars().skip(1))
        .collect()
}

#[test]
fn test_capitalize() {
    assert_eq!(capitalize("hello"), "Hello".to_owned());
}
