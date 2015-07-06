use dao::Value;
use database::SqlOption;
use std::fmt;

/// sql fragment
/// use this for writing SQL statements
pub struct SqlFrag{
    pub sql:String,
    pub params:Vec<Value>,
    pub sql_options: Vec<SqlOption>,
}

impl fmt::Display for SqlFrag{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.sql);
        let mut do_comma = false;
        write!(f, "[");
        for param in &self.params{
            if do_comma {write!(f, ", "); } else {do_comma = true;}
            write!(f, "{}", param);
        }
        write!(f, "]")
    }
}

impl SqlFrag{
    
    #[inline]
    pub fn new(sql_options: Vec<SqlOption>)->Self{
        SqlFrag{sql:String::new(), params:vec![], sql_options: sql_options}
    }
    
    #[inline]
    pub fn append(&mut self, str:&str)->&mut Self{
        self.sql.push_str(str);
        self
    }
    
    #[inline]
    pub fn appendln(&mut self, str:&str)->&mut Self{
        self.append(str);
        self.ln()
    }
    
    #[inline]
    pub fn tab(&mut self)->&mut Self{
        self.append("    ")
    }
    #[inline]
    pub fn tabs(&mut self, n:u32)->&mut Self{
        for _ in 0..n{
            self.tab();
        }
        self
    }
    #[inline]
    pub fn ln(&mut self)->&mut Self{
        self.append("\n")
    }
    #[inline]
    pub fn ln_tab(&mut self)->&mut Self{
        self.ln();
        self.tab()
    }
    #[inline]
    pub fn ln_tabs(&mut self, n:u32)->&mut Self{
        self.ln();
        self.tabs(n)
    }
    #[inline]
    pub fn comma(&mut self)->&mut Self{
        self.append(",")
    }
    #[inline]
    pub fn sp(&mut self)->&mut Self{
        self.append(" ")
    }
    #[inline]
    pub fn commasp(&mut self)->&mut Self{
        self.comma().sp()
    }
    
    #[inline]
    pub fn comment(&mut self, comment: &str)->&mut Self{
        self.append("-- ");
        self.append(comment)
    }
    ///append parameter including the needed sql keywords
    pub fn parameter(&mut self, param:Value){
        self.params.push(param);
        if self.sql_options.contains(&SqlOption::UsesNumberedParam){
            let numbered_param = format!("${} ", self.params.len());
            self.append(&numbered_param);
        }
        else if self.sql_options.contains(&SqlOption::UsesQuestionMark){
            self.append("?");
        }
    }
    
}
