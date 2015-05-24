#[derive(Debug)]
pub struct Writer{
    pub src:String,
}

impl Writer{
    
    pub fn new()->Self{
        Writer{src:String::new()}
    }
    
    pub fn append(&mut self, str:&str)->&mut Self{
        self.src.push_str(str);
        self
    }
    
    pub fn appendln(&mut self, str:&str)->&mut Self{
        self.append(str);
        self.ln()
    }
    
    pub fn tab(&mut self)->&mut Self{
        self.append("\t")
    }
    pub fn ln(&mut self)->&mut Self{
        self.append("\n")
    }
    pub fn comma(&mut self)->&mut Self{
        self.append(",")
    }
}
