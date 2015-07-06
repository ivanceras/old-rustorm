pub mod postgres;
pub mod sqlite;

pub use self::postgres::Postgres;
pub use self::sqlite::Sqlite;

use database::Database;
use database::DatabaseDDL;




pub enum Platform{
    Postgres(Postgres),
    Sqlite(Sqlite),
    Oracle,
    Mysql,
}

impl Platform{
    
    pub fn as_ref(&self)->&Database{
        match *self{
            Platform::Postgres(ref pg) => pg,
            Platform::Sqlite(ref lite) => lite,
            _ => panic!("others not yet..")
        }
    }
    pub fn as_ddl(&self)->&DatabaseDDL{
        match *self{
            Platform::Postgres(ref pg) => pg,
            Platform::Sqlite(ref lite) => lite,
            _ => panic!("others not yet..")
        }
    }
    
}

impl Drop for Platform {
    fn drop(&mut self) {
        println!("Warning: Dropping a connection is expensive, please return this to the pool");
    }
}
