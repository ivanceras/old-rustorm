use dao::Type;

/// generic string writer
#[derive(Debug)]
pub struct Writer{
    pub src:String,
}

impl Writer{
    
    #[inline]
    pub fn new()->Self{
        Writer{src:String::new()}
    }
    
    #[inline]
    pub fn append(&mut self, str:&str)->&mut Self{
        self.src.push_str(str);
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
        self.append("//");
        self.append(comment)
    }
}

/// SqlOption
pub enum SqlOption{
    /// use the numbered parameters, as the case with rust-postgres
    UseNumberedParam,
    /// sqlite, jdbc
    UseQuestionMark,
}

/// sql fragment
/// TODO; unify the functions with the generic string writer
pub struct SqlFrag{
    pub sql:String,
    pub params:Vec<Type>,
    pub sql_option:SqlOption,
}

impl SqlFrag{
    
    #[inline]
    pub fn new()->Self{
        SqlFrag{sql:String::new(), params:vec![], sql_option:SqlOption::UseNumberedParam}
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
    pub fn parameter(&mut self, param:Type){
        self.params.push(param);
        match self.sql_option{
            SqlOption::UseNumberedParam => {
                let numbered_param = format!("${} ", self.params.len());
                self.append(&numbered_param);
            },
            SqlOption::UseQuestionMark => {
                self.append("?");
            }
        }
    }
    
}
