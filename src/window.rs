use table::Table;
use std::collections::HashMap;

/// visual presentation of column of a table
///directly corresponds to a column of a table
pub struct Field{
    pub name:String,
    
    /// the column name this field corresponds to
    pub column:String,
    /// this will be the bases of how to display the field in the UI
    /// such as Date will be displayed with a date picker
    pub data_type:String,
    
    ///reference needed such as if it is a color, person, contact numer, etc.
    /// example values: Table, List, Amount, Quantity, Date
    /// its up for the UI to intelligently display the field on the client side
    /// such as displaying a country with the flag at the side of the name
    /// person lookup with a search field and a person logo
    pub reference:String,
    /// short concise description of this field
    pub description:Option<String>,
    ///more information about this field, help text for users
    pub info:Option<String>,
    /// whether or not this field, contributes to the records identity
    /// the distinctive display name will be derived from fields marked a identifier
    pub is_identifer:bool,
    /// whether or not this field, will have to be included in the searching of records
    /// same as is_selection_column
    pub include_in_search:bool,
    /// is the field mandatory for the user to fill up.
    pub is_mandatory:bool,
    ///ordering of the fields, when displayed on the UI
    pub seq_no:f32,
    ///should be same line or no
    pub is_same_line:bool,
    pub is_displayed:bool,
    ///whether or not this field is editable
    pub is_readonly:bool,
    ///whether or not possible matching values will be displayed while the user is typing
    pub is_autocomplete:bool,
    
    /// a dyanmic code to determine whether this field will be displayed or not
    /// example $active == true
    pub display_logic:Option<String>,
    /// the preffered length of the text this field will occupy 
    pub display_length:Option<String>, 
    /// this can be displayed in the placeholder value, by evaluating the defaults
    /// such as now() and try to come up with a value, which is the current date
    pub default_value:Option<String>,
}


/// Tab is a visual presentation of a table
/// [FIXME] how are the filters, joins expressed between tab to other tabs
/// When a user open a tab, a list of 10 values will be listed
pub struct Tab{
    pub name:String,
    /// extension tables' fields will have to be listed along side
    /// with the main tables
    pub is_extension:bool,
    /// has one tables will have 1 field, displaying the identifiers of the referred tables
    pub is_has_one:bool,
    /// can be from direct referring table or indirect linker table
    pub is_has_many:bool,
    pub is_direct:bool,
    pub description:Option<String>,
    /// more information of this tab
    pub info:Option<String>,
    ///which table does this tab corresponds to
    pub table:String,
    pub schema:String,
    pub fields:Vec<Field>,
    ///other children tabs
    pub tabs:Option<Vec<Tab>>,
    ///optional logo/emblem for the user to uniquely identify this tab.
    ///its color pallete can be used to be as a mini theme of the window itself
    /// in order for the user to have distinct sense on each of the windows, which has
    /// more or less similar set of fields and styles.
    /// a scaled version of the logo can be added to the icon
    pub logo:Option<String>,
    /// a small image to be embedded on the toolbar or tooltip when used in a referred lookup
    pub icon:Option<String>,
    /// an optional page size of when paging records on this tab
    /// items_per_page
    pub page_size:Option<u32>,
    ///default ordering of (columns, ASC | DESC)
    pub default_order:HashMap<String, bool>,
}

///directly correspond to a table, no need for tabs
pub struct Window{
    ///name of the window
    pub name:String,
    pub description:Option<String>,
    ///main tab, must have at least 1
    /// more helpful information about this window
    pub info:Option<String>,
    pub tab:Tab,
}

/// build windows from a set of tables
/// 
pub fn extract_windows(tables:&Vec<Table>){
    let mut window_tables = Vec::new();
    let all_extension_tables = get_all_extension_tables(tables);
    for t in tables{
        if t.is_linker_table(){
            println!("NOT a Window: {} <<-linker table", t.name);
        }
        else{   
            if t.is_owned(tables){
                println!("OWNED table: {}", t.name);
            }
            else{
                if all_extension_tables.contains(&t){
                    println!("EXTENSION table: {}", t);
                }
                else{
                    println!("{}", t.name);
                    window_tables.push(t);
                    for (col, has1) in t.referred_tables(tables){
                        println!("\t has one: {}({}) {} condensed: {}",col.displayname(), col.name, has1.name, col.condense_name());
                    }
                    for ext in t.extension_tables(tables){
                        println!("\t ext tab: {}", ext.name);
                    }
                    for (has_many,_) in t.referring_tables(tables){
                        if !has_many.is_linker_table(){
                            println!("\t has many direct: {}", has_many.name);
                        }else{
                            //println!("\t has many direct: {} <---- but is a linker table, so no!", has_many.name);
                        }
                    }
                    for (has_many,linker) in t.indirect_referring_tables(tables){
                        println!("\t has many: {} via {}",has_many.name, linker.name);
                    }
                }
            }
        }
    }
    // all other tables can also have their own windows, but will not be displayed on the main menu.
    println!("Final list of main window tables");
    for w in window_tables{
        println!("{}", w);
    }
}

fn get_all_extension_tables(tables:&Vec<Table>)->Vec<&Table>{
    let mut all_extension_tables = Vec::new();
    for t in tables{
        for ext in t.extension_tables(tables){
            if !all_extension_tables.contains(&ext){
                println!("extension table: {}", ext);
                all_extension_tables.push(ext);
            }
        }
    }
    all_extension_tables

}