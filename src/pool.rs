use r2d2::Pool;
use r2d2_postgres::{PostgresConnectionManager};
use r2d2::Config;
use postgres::SslMode;
use config::DbConfig;
use database::{Database, DatabaseDDL, DatabaseDev};
use platform::Postgres;
use platform::Sqlite;
use platform::Mysql;
use mysql::conn::pool::{MyPool, MyPooledConn};
use mysql::conn::MyOpts;


/// the sql builder for each of the database platform
pub enum Platform{
    Postgres(Postgres),
    Sqlite(Sqlite),
    Oracle,
    Mysql(Mysql),
}


impl Platform{
    
    pub fn as_ref(&self)->&Database{
        match *self{
            Platform::Postgres(ref pg) => pg,
            Platform::Sqlite(ref lite) => lite,
            _ => panic!("others not yet..")
        }
    }
    pub fn as_ref_mut(&mut self)->&mut Database{
        match *self{
            Platform::Postgres(ref mut pg) => pg,
            Platform::Sqlite(ref mut lite) => lite,
            Platform::Mysql(ref mut my) => my,
            _ => panic!("others not yet..")
        }
    }
    pub fn as_ddl(&self)->&DatabaseDDL{
        match *self{
            Platform::Postgres(ref pg) => pg,
            Platform::Sqlite(ref lite) => lite,
            Platform::Mysql(ref my) => my,
            _ => panic!("others not yet..")
        }
    }
    pub fn as_ddl_mut(&mut self)->&mut DatabaseDDL{
        match *self{
            Platform::Postgres(ref mut pg) => pg,
            Platform::Sqlite(ref mut lite) => lite,
            Platform::Mysql(ref mut my) => my,
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
    Mysql(MyPool),
}

impl ManagedPool{
    
    /// initialize the pool
    /// TODO: return result instead of unwrap
    pub fn init(url: &str, pool_size: usize)->Self{
        let config = DbConfig::from_url(url);
        let platform:&str = &config.platform;
        match platform{
            "postgres" => {
                let manager = PostgresConnectionManager::new(url, SslMode::None).unwrap();
                let config = Config::builder().pool_size(pool_size as u32).build();
                let pool = Pool::new(config, manager).unwrap();
                ManagedPool::Postgres(pool)
            }
            "sqlite" => {
                panic!("sqlite soon!");
            }
            "mysql" => {
                let config = DbConfig::from_url(url);
                let opts = MyOpts {
                    user: config.username,
                    pass: config.password,
                    db_name: Some(config.database),
                    tcp_addr: Some(config.host.unwrap().to_string()),
                    tcp_port: config.port.unwrap_or(3306),
                    ..Default::default()
                };
                let pool = MyPool::new_manual(0, pool_size, opts).unwrap();
                ManagedPool::Mysql(pool)
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
            ManagedPool::Mysql(ref pool) => {
                let conn = pool.get_conn();//the connection is created here
                match conn{
                    Ok(conn) => {
                        let my = Mysql::with_connection(conn.unwrap());
                        Ok(Platform::Mysql(my))
                    },
                    Err(e) => {
                        Err(format!("Unable to connect {}", e))
                    }
                }
            },
            _ => Err("Any other database is not yet supported".to_string())
        }
    }
}
