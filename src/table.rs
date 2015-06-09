use std::fmt;
use writer::Writer;

#[derive(Debug, Clone, PartialEq)]
pub struct Foreign{
    pub schema:String,
    pub table:String,
    pub column:String,
}

impl Foreign{
    /// get the definition express in string code
    pub fn to_source_code(&self)->String{
        let mut w = Writer::new();
        w.ln();
        w.tabs(6);
        w.append("Foreign{");
        w.ln();
        w.tabs(7);
        w.append("schema:");
        w.append(&format!("\"{}\".to_string(),",self.schema));
        w.ln();
        w.tabs(7);
        w.append("table:");
        w.append(&format!("\"{}\".to_string(),",self.table));
        w.ln();
        w.tabs(7);
        w.append("column:");
        w.append(&format!("\"{}\".to_string(),",self.column));
        w.ln();
        w.tabs(6);
        w.append("}");
        w.src
    }
}

#[derive(Debug, Clone)]
pub struct Column{
    pub name:String,
    /// the generic data type, ie: u32, f64, string
    pub data_type:String,
    /// the database data type of this column, ie: int, numeric, character varying
    pub db_data_type:String,
    pub is_primary:bool,
    pub is_unique:bool,
    pub default:Option<String>,
    pub comment:Option<String>,
    pub not_null:bool,
    pub foreign:Option<Foreign>,
    ///determines if the column is inherited from the parent table
    pub is_inherited:bool,
}

impl Column{

    fn is_keyword(str:&str)->bool{
        let keyword = ["type", "yield", "macro"];
        keyword.contains(&str)
    }


    ///some column names may be a rust reserve keyword, so have to correct them
    pub fn corrected_name(&self)->String{
        if Self::is_keyword(&self.name){
            println!("Warning: {} is rust reserved keyword", self.name);
            return format!("{}_",self.name);
        }
        self.name.to_string()
    }
    
     pub fn displayname(&self)->String{
         let clean_name = self.clean_name();
         clean_name.replace("_", " ")
    }
    
    /// presentable display names, such as removing the ids if it ends with one
    fn clean_name(&self)->String{
        if self.name.ends_with("_id"){
            return self.name.trim_right_matches("_id").to_string();
        }
        self.name.to_string()
    }
    
    /// shorten, compress the name based on the table it points to
    /// parent_organization_id becomes parent
    pub fn condense_name(&self)->String{
        let clean_name = self.clean_name();
        if self.foreign.is_some(){
            let foreign = &self.foreign.clone().unwrap();
            if clean_name.len() > foreign.table.len(){
                return clean_name
                        .trim_right_matches(&foreign.table)
                        .trim_right_matches("_")
                        .to_string();
            }
        }
        clean_name
    }
    
    /// get the column definition of the code
    pub fn to_column_def_source_code(&self)->String{
        let mut w = Writer::new();
        w.ln();
        w.tabs(4);
        w.append("Column{");
        w.ln();
        w.tabs(5);
        w.append("name:");
        w.append(&format!("\"{}\".to_string(),",self.name));
        w.ln();
        w.tabs(5);
        w.append("data_type:");
        w.append(&format!("\"{}\".to_string(),",self.data_type));
        w.ln();
        w.tabs(5);
        w.append("db_data_type:");
        w.append(&format!("\"{}\".to_string(),",self.db_data_type));
        w.ln();
        w.tabs(5);
        w.append("is_primary:");
        w.append(&format!("{}, ",self.is_primary));
        w.append("is_unique:");
        w.append(&format!("{}, ",self.is_unique));
        w.append("not_null:");
        w.append(&format!("{}, ",self.not_null));
        w.append("is_inherited:");
        w.append(&format!("{}, ",self.is_inherited));
        w.ln();
        w.tabs(5);
        w.append("default:");
        if self.default.is_some(){
            w.append(&format!("Some(\"{}\".to_string()),", &self.default.clone().unwrap()));
        }else{
            w.append("None,");
        }
        w.ln();
        w.tabs(5);
        w.append("comment:");
        if self.comment.is_some(){
            w.append(&format!("Some(\"{}\".to_string()),", &self.comment.clone().unwrap().replace("\"","\\\"")));
        }else{
            w.append("None,");
        }
        w.ln();
        w.tabs(5);
        w.append("foreign:");
        if self.foreign.is_some(){
            w.append(&format!("Some({}),", &self.foreign.clone().unwrap().to_source_code()));
        }else{
            w.append("None,");
        }
        w.ln();
        w.tabs(4);
        w.append("}");
        w.src
    }


}


impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialEq for Column{
    fn eq(&self, other: &Self) -> bool{
        self.name == other.name
     }

    fn ne(&self, other: &Self) -> bool {
        self.name != other.name
    }
}


pub trait IsTable{
    fn table()->Table;
}



#[derive(Debug)]
#[derive(Clone)]
pub struct Table{

    ///which schema this belongs
    pub schema:String,

    ///the table name
    pub name:String,

    ///the parent table of this table when inheriting (>= postgresql 9.3)
    /// [FIXME] need to tell which schema this parent table belongs
    /// there might be same table in different schemas
    pub parent_table:Option<String>,

    ///what are the other table that inherits this
    /// [FIXME] need to tell which schema this parent table belongs
    /// there might be same table in different schemas
    pub sub_table:Option<Vec<String>>,

    ///comment of this table
    pub comment:Option<String>,

    ///columns of this table
    pub columns:Vec<Column>,

}
impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialEq for Table{
    fn eq(&self, other: &Self) -> bool{
        self.name == other.name && self.schema == other.schema
     }

    fn ne(&self, other: &Self) -> bool {
        self.name != other.name || self.schema != other.schema
    }
}


impl Table{
    
    /// return the long name of the table using schema.table_name
    pub fn long_name(&self)->String{
        format!("{}.{}", self.schema, self.name)
    }
    
    /// determine if this table has a colum named
    pub fn has_column_name(&self, column:&String)->bool{
        for c in &self.columns{
            if c.name == column.clone(){
                return true;
            }
        }
        false
    }

    /// return all the primary columns of this table
    pub fn primary_columns(&self)->Vec<&Column>{
        let mut primary_columns = Vec::new();
        for c in &self.columns{
            if c.is_primary{
                primary_columns.push(c);
            }
        }
        primary_columns.sort_by(|a, b| a.name.cmp(&b.name));
        primary_columns
    }
    
    /// return all the columns of this table excluding the inherited columns
    pub fn uninherited_columns(&self)->Vec<&Column>{
        let mut included = Vec::new();
        let mut uninherited_columns = Vec::new();
        for c in &self.columns{
            if !c.is_inherited && !included.contains(&&c.name){
                uninherited_columns.push(c);
                included.push(&c.name);
            }
        }
        uninherited_columns.sort_by(|a, b| a.name.cmp(&b.name));
        uninherited_columns
    }

    /// return all the inherited columns
    pub fn inherited_columns(&self)->Vec<&Column>{
        let mut included = Vec::new();
        let mut inherited_columns = Vec::new();
        for c in &self.columns{
            if c.is_inherited && !included.contains(&&c.name){
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
    pub fn is_primary(&self, column_name:&str)->bool{
        for p in self.primary_columns(){
            if p.name == column_name {
                return true;
            }
        }
        false
    }

    /// return all the unique keys of this table
    pub fn unique_columns(&self)->Vec<&Column>{
        let mut unique_columns = Vec::new();
        for c in &self.columns{
            if c.is_unique{
                unique_columns.push(c);
            }
        }
        unique_columns.sort_by(|a, b| a.name.cmp(&b.name));
        unique_columns
    }

    pub fn foreign_columns(&self)->Vec<&Column>{
        let mut columns = Vec::new();
        for c in &self.columns{
            if c.foreign.is_some(){
                columns.push(c);
            }
        }
        columns.sort_by(|a, b| a.name.cmp(&b.name));
        columns
    }

    /// return the first match of table name regardless of which schema it belongs to.
    /// get the table definition using the table name from an array of table object
    /// [FIXME] Needs to have a more elegant solution by using HashMap
    pub fn get_table<'a>(table_name:&str, tables: &'a Vec<Table>)->Option<&'a Table>{
        for t in tables{
            if t.name == table_name{
                return Some(t);
            }
        }
        None
    }



    /// get all the tables that is referred by this table
    /// get has_one
    pub fn referred_tables<'a>(&'a self, tables:&'a Vec<Table>)->Vec<(&'a Column, &'a Table)>{
        let mut referred_tables = Vec::new();
        for c in &self.columns{
            if c.foreign.is_some(){
                let ftable_name = &c.foreign.clone().unwrap().table;
                let ftable = Self::get_table(ftable_name, tables).unwrap();
                referred_tables.push((c, ftable));
            }
        }
        referred_tables
    }

    /// has_many_direct
    /// get all other tables that is refering to this table
    /// when any column of a table refers to this table
    /// get_has_many
    pub fn referring_tables<'a>(&self, tables: &'a Vec<Table>)->Vec<(&'a Table, &'a Column)>{
        let mut referring = Vec::new();
        for t in tables{
            for c in &t.columns{
                if c.foreign.is_some(){
                    if &self.name == &c.foreign.clone().unwrap().table{
                        referring.push((t, c));
                    }
                }
            }
        }
        referring
    }
    
    ///determine if this table is a linker table
    /// FIXME: make sure that there are 2 different tables referred to it
    pub fn is_linker_table(&self)->bool{
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
    pub fn is_owned(&self, tables: &Vec<Table>)->bool{
        let has_one = self.referred_tables(tables);
        let has_many = self.referring_tables(tables);
        has_one.len() == 1 && has_many.len() == 0
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
    pub fn indirect_referring_tables<'a>(&self, tables: &'a Vec<Table>)->Vec<(&'a Table, &'a Table)>{
        let mut indirect_referring_tables = Vec::new();
        for (rt, column) in self.referring_tables(tables){
            let rt_pk = rt.primary_columns();
            let rt_fk = rt.foreign_columns();
            let rt_uc = rt.uninherited_columns();
            if rt_pk.len() == 2 && rt_fk.len() == 2 && rt_uc.len() == 2 {
                //println!("{} is a candidate linker table for {}", rt.name, self.name);
                let ref_tables = rt.referred_tables(tables);
                let (_, t0) = ref_tables[0];
                let (_, t1) = ref_tables[1];
                let mut other_table;
                //if self.name == t0.name && self.schema == t0.schema{
                if self == t0 {
                    other_table = t1;
                }
                else{
                    other_table = t0;
                }
                let mut cnt = 0;
                for fk in &rt_fk{
                    if self.is_foreign_column_refer_to_primary_of_this_table(fk){
                        cnt += 1;
                    }
                    if other_table.is_foreign_column_refer_to_primary_of_this_table(fk){
                        cnt += 1;
                    }
                }
                
                if cnt == 2{
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
    pub fn extension_tables<'a>(&self, tables: &'a Vec<Table>)->Vec<&'a Table>{
        let mut extension_tables = Vec::new();
        for (rt, _) in self.referring_tables(tables){
            let pkfk = rt.primary_and_foreign_columns();
            let rt_pk = rt.primary_columns();
            //if the referring tables's foreign columns are also its primary columns
            //that refer to the primary columns of this table
            //then that table is just an extension table of this table
            if rt_pk == pkfk && pkfk.len() > 0 {
                //if all fk refer to the primary of this table
                if self.are_these_foreign_column_refer_to_primary_of_this_table(&pkfk){
                    extension_tables.push(rt);
                }
            }
        }
        extension_tables
    }
    
    /// returns only columns that are both primary and foreign
    /// FIXME: don't have to do this if the function getmeta data has merged this.
    fn primary_and_foreign_columns(&self)->Vec<&Column>{
        let mut both = Vec::new();
        let pk = self.primary_columns();
        let fk = self.foreign_columns();
        for f in fk{
            if pk.contains(&f){
                //println!("{}.{} is both primary and foreign", self.name, f.name);
                both.push(f);
            }
        }
        both
    }
    
    fn is_foreign_column_refer_to_primary_of_this_table(&self, fk:&Column)->bool{
        if fk.foreign.is_some(){
            let foreign = fk.foreign.clone().unwrap();
            let table = foreign.table;
            let schema = foreign.schema;
            let column = foreign.column;
            if self.name == table && self.schema == schema && self.is_primary(&column){
                return true;
            }
        }
        false
    }
        
    fn are_these_foreign_column_refer_to_primary_of_this_table(&self, rt_fk:&Vec<&Column>)->bool{
        let mut cnt = 0;
        for fk in rt_fk{
            if self.is_foreign_column_refer_to_primary_of_this_table(fk){
                cnt += 1;
            }
        }
        cnt == rt_fk.len()
    }



    /// capitalize the first later, if there is underscore remove it then capitalize the next letter
    pub fn struct_name(&self)->String{
        let mut struct_name = String::new();
        for i in self.name.split('_'){
                struct_name.push_str(&capitalize(i));
        }
        struct_name
    }
    
    /// get the display name of this table
    /// product_availability -> Product Availability
    pub fn displayname(&self)->String{
        let mut display_name = String::new();
        for i in self.name.split('_'){
            display_name.push_str(&capitalize(i));
            display_name.push_str(" ");
        }
        display_name.trim().to_string()
    }
    
    /// get a shorter display name of a certain table
    /// when being refered to this table
    /// example product.product_availability -> Availability
    /// user.user_info -> Info
    pub fn concise_name(&self, table:&Table)->String{
        if self.name.len() > table.name.len(){
            let mut concise_name = String::new();
            for i in self.name.split('_'){
                if table.name != i{
                    concise_name.push_str(&capitalize(i));
                    concise_name.push_str(" ");
                }
            }
            return concise_name.trim().to_string()    
        }else{
            return self.displayname();
        }
    }
    /// build a source code which express it self as a table object
    /// which is a meta definition of the struct itself
    pub fn to_tabledef_source_code(&self)->String{
        let mut w = Writer::new();
        w.ln();
        w.tabs(2);
        w.append("Table{");
        w.ln();
        w.tabs(3);
        w.append("schema:");
        w.append(&format!("\"{}\".to_string(),", self.schema));
        w.ln();
        w.tabs(3);
        w.append("name:");
        w.append(&format!("\"{}\".to_string(),", self.name));
        w.ln();
        w.tabs(3);
        w.append("parent_table:");
        if self.parent_table.is_some(){
            w.append(&format!("Some(\"{}\".to_string()),", &self.parent_table.clone().unwrap()));
        }else{
            w.append("None,");
        }
        w.ln();
        w.tabs(3);
        w.append("sub_table:");
        if self.sub_table.is_some(){
            let sub_table = self.sub_table.clone().unwrap();
            w.append("Some(");
            w.append("vec![");
            for s in sub_table{
                w.append(&format!("\"{}\".to_string(),",s));
            }
            w.append("]),");
        }else{
            w.append("None,");
        }
        w.ln();
        w.tabs(3);
        w.append("comment:");
        if self.comment.is_some(){
            w.append(&format!("Some(\"{}\".to_string()),", &self.comment.clone().unwrap().replace("\"","\\\"")));
        }else{
            w.append("None,");
        }
        w.ln();
        w.tabs(3);
        w.append("columns:");
        w.ln();
        w.tabs(3);
        w.append("vec![");
        for c in &self.columns{
            w.append(&c.to_column_def_source_code());
            w.append(",");
        }
        w.ln();
        w.tabs(3);
        w.append("],");
        w.ln();
        w.tabs(2);
        w.append("}");
        w.src
    }


}


fn capitalize(str:&str)->String{
     str.chars().take(1)
         .flat_map(char::to_uppercase)
        .chain(str.chars().skip(1))
        .collect()
}

#[test]
fn test_capitalize(){
    assert_eq!(capitalize("hello"), "Hello".to_string());
}
