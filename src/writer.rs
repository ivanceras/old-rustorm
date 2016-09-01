use dao::Value;
use database::{SqlOption, BuildMode};
use std::fmt;

/// sql fragment
/// use this for writing SQL statements
pub struct SqlFrag {
    pub sql: String,
    pub params: Vec<Value>,
    pub sql_options: Vec<SqlOption>,
    pub build_mode: BuildMode,
}

impl fmt::Display for SqlFrag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.sql));
        let mut do_comma = false;
        try!(write!(f, "["));
        for param in &self.params {
            if do_comma {
                try!(write!(f, ", "));
            } else {
                do_comma = true;
            }
            try!(write!(f, "{}", param));
        }
        write!(f, "]")
    }
}

impl SqlFrag {
    #[inline]
    pub fn new(sql_options: Vec<SqlOption>, build_mode: BuildMode) -> Self {
        SqlFrag {
            sql: String::new(),
            params: vec![],
            sql_options: sql_options,
            build_mode: build_mode,
        }
    }

    #[inline]
    pub fn append(&mut self, str: &str) -> &mut Self {
        self.sql.push_str(str);
        self
    }

    #[inline]
    pub fn appendln(&mut self, str: &str) -> &mut Self {
        self.append(str);
        self.ln()
    }

    #[inline]
    pub fn tab(&mut self) -> &mut Self {
        self.append("    ")
    }
    #[inline]
    pub fn tabs(&mut self, n: u32) -> &mut Self {
        for _ in 0..n {
            self.tab();
        }
        self
    }
    #[inline]
    pub fn ln(&mut self) -> &mut Self {
        self.append("\n")
    }
    #[inline]
    pub fn ln_tab(&mut self) -> &mut Self {
        self.ln();
        self.tab()
    }
    #[inline]
    pub fn ln_tabs(&mut self, n: u32) -> &mut Self {
        self.ln();
        self.tabs(n)
    }
    #[inline]
    pub fn comma(&mut self) -> &mut Self {
        self.append(",")
    }
    #[inline]
    pub fn sp(&mut self) -> &mut Self {
        self.append(" ")
    }
    #[inline]
    pub fn spaces(&mut self, n: i32) -> &mut Self {
        for _ in 0..n {
            self.sp();
        }
        self
    }
    /// river is the line in the SQL statment which makes it more readable
    /// * http://www.sqlstyle.guide/
    /// river size is 9, `RETURNING`
    #[inline]
    fn river(&mut self, str: &str) -> &mut Self {
        let river_size: i32 = 9;
        let trim = str.trim();
        let diff: i32 = river_size - trim.len() as i32;
        if diff > 0 {
            self.spaces(diff);
        }
        self.append(trim);
        self.sp()
    }
    /// write the string, aligning to the left side of the middle space (river)
    #[inline]
    pub fn left_river(&mut self, str: &str) -> &mut Self {
        self.ln();
        self.river(str)
    }
    /// write the string, aligning to the right side of the middle space (river), leaving the left with empty string
    #[inline]
    pub fn right_river(&mut self, str: &str) -> &mut Self {
        self.ln();
        self.river("");
        self.append(str)
    }

    #[inline]
    pub fn commasp(&mut self) -> &mut Self {
        self.comma().sp()
    }

    #[inline]
    pub fn comment(&mut self, comment: &str) -> &mut Self {
        self.append("-- ");
        self.append(comment)
    }
    /// append parameter including the needed sql keywords
    pub fn parameter(&mut self, param: Value) {
        match self.build_mode {
            BuildMode::Standard => {
                self.params.push(param);
                if self.sql_options.contains(&SqlOption::UsesNumberedParam) {
                    let numbered_param = format!("${} ", self.params.len());
                    self.append(&numbered_param);
                } else if self.sql_options.contains(&SqlOption::UsesQuestionMark) {
                    self.append("?");
                }
            }
            BuildMode::Debug => {
                // use fmt::Display
                self.append(&format!("{}",&param));
            }
        }
    }
}
