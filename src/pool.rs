use r2d2::Pool;
use r2d2_postgres::{PostgresConnectionManager};
use r2d2::Config;
use postgres::SslMode;
use config::DbConfig;
use database::{Database, DatabaseDDL, DatabaseDev};
use platform::Postgres;
use platform::Sqlite;


/// the sql builder for each of the database platform
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
    
    pub fn as_dev(&self)->&DatabaseDev{
        match *self{
            Platform::Postgres(ref pg) => pg,
            _ => panic!("others not yet..")
        }
    }
}

pub enum ManagedPool{
    Postgres(Pool<PostgresConnectionManager>),
    Sqlite,
    Oracle,
    Mysql,
}

impl ManagedPool{
    
    /// initialize the pool
    pub fn init(url: &str, pool_size: u32)->Self{
        let config = DbConfig::from_url(url);
        let platform:&str = &config.platform;
        match platform{
            "postgres" => {
                    let manager = PostgresConnectionManager::new(url, SslMode::None).unwrap();
                    let config = Config::builder().pool_size(pool_size).build();
                    let pool = Pool::new(config, manager).unwrap();
                    ManagedPool::Postgres(pool)
            }
            "sqlite" => {
                panic!("sqlite soon!");
            }
            _ => panic!("not yet")
        }
    }
    
    /// a conection is created here
    pub fn connect(&self)->Result<Platform, String>{
        match *self{
            ManagedPool::Postgres(ref pool) => {
                let conn = pool.get();//the connection is created here
                match conn{
                    Ok(conn) => {
                        let pg = Postgres::with_pooled_connection(conn);
                        Ok(Platform::Postgres(pg))
                    },
                    Err(e) => {
                        Err(format!("Unable to connect {}", e))
                    }
                }
            },
            ManagedPool::Sqlite => {
                Err("Sqlite is not supported yet".to_string())
            },
            _ => Err("Any other database is not yet supported".to_string())
        }
    }
}
