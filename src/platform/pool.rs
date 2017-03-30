#[allow(unused)]
use r2d2::Pool;
#[allow(unused)]
use r2d2::Config;
use config::DbConfig;
use database::{Database, DatabaseDDL, DatabaseDev};
#[cfg(feature = "postgres")]
use platform::Postgres;
#[cfg(feature = "postgres")]
use r2d2_postgres::PostgresConnectionManager;
#[cfg(feature = "postgres")]
use r2d2_postgres::SslMode;
#[cfg(feature = "sqlite")]
use platform::Sqlite;
#[cfg(feature = "mysql")]
use platform::Mysql;
#[cfg(feature = "mysql")]
use mysql::conn::pool::MyPool;
#[cfg(feature = "mysql")]
use mysql::conn::MyOpts;
use database::DbError;
use std::ops::Deref;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[cfg(feature = "sqlite")]
use r2d2_sqlite::SqliteConnectionManager;


lazy_static! {
    pub static ref DB_POOL: Arc<RwLock<HashMap<PoolConfig, ManagedPool>>> = 
            Arc::new(RwLock::new(HashMap::new()));
}

// 1 pool per connection name
// check the connection name supplied
// has the same db_url configuration
// different connection name
// will have different connection pool
#[derive(PartialEq,Eq)]
#[derive(Hash)]
#[derive(Clone)]
pub struct PoolConfig{
    connection_name: String, 
    db_url: String,
    pool_size: u32,
}


/// no connection name supplied
/// pool size is 10;
#[cfg(any(feature = "postgres",feature = "sqlite", feature ="mysql"))]
pub fn db_with_url(db_url: &str) -> Result<Platform, DbError> {
    let config = PoolConfig{
        connection_name: "GLOBAL".to_string(),
        db_url: db_url.to_string(),
        pool_size: 10
    };
    db_with_config(&config)
}

#[cfg(any(feature = "postgres",feature = "sqlite", feature ="mysql"))]
pub fn test_connection(db_url: &str) -> Result<(), DbError>{
    let config = DbConfig::from_url(db_url);
    match config {
        Some(config) => {
            let platform: &str = &config.platform;
            match platform {
                #[cfg(feature = "postgres")]
                "postgres" => {
                    ::platform::postgres::establish_connection(db_url)?;
                    Ok(())
                }

                #[cfg(feature = "sqlite")]
                "sqlite" => {
                    ::platform::sqlite::establish_connection(db_url)?;
                    Ok(())
                }
                #[cfg(feature = "mysql")]
                "mysql" => {
                    ::platform::mysql::establish_connection(&config)?;
                    Ok(())
                }

                _ => unimplemented!(),
            }
        }
        None => {
            println!("Unable to parse url");
            Err(DbError::new("Error parsing url"))
        }
    }
}

#[cfg(any(feature = "postgres",feature = "sqlite", feature ="mysql"))]
pub fn db_with_config(config: &PoolConfig) -> Result<Platform, DbError> {
    let has_pool = DB_POOL.read().unwrap().get(&config).is_some();
    if has_pool{
        DB_POOL.read().unwrap().get(&config).unwrap().connect()
    }else{
        create_new(&config)
    }
}

/// creates a new ManagedPool for this database platform
#[cfg(any(feature = "postgres",feature = "sqlite", feature ="mysql"))]
fn create_new(config: &PoolConfig) -> Result<Platform, DbError> {
    println!("not an existing pool, creating one");
    let pool = ManagedPool::init(&config.db_url, config.pool_size as usize)?;
    let conn = pool.connect();
    println!("inserting to the Pool");
    DB_POOL.write().unwrap().insert(config.clone(), pool);
    println!("inserted!");
    conn
}

/// the sql builder for each of the database platform
pub enum Platform {
    #[cfg(feature = "postgres")]
    Postgres(Postgres),
    #[cfg(feature = "sqlite")]
    Sqlite(Sqlite),
    #[cfg(feature = "mysql")]
    Mysql(Mysql),
}

impl Platform {

    /// create a postgresql
    /// database instance
    /// without doing a connection
    #[cfg(feature = "postgres")]
    pub fn pg() -> Self {
       Platform::Postgres(Postgres::new()) 
    }

    #[cfg(feature = "sqlite")]
    pub fn sqlite() -> Self {
        Platform::Sqlite(Sqlite::new())
    }

    #[cfg(feature = "mysql")]
    pub fn mysql() -> Self {
        Platform::Mysql(Mysql::new())
    }

    pub fn as_ref(&self) -> &Database {
        match *self {
            #[cfg(feature = "postgres")]
            Platform::Postgres(ref pg) => pg,
            #[cfg(feature = "sqlite")]
            Platform::Sqlite(ref lite) => lite,
            #[cfg(feature = "mysql")]
            Platform::Mysql(ref my) => my,
        }
    }

    pub fn as_ddl(&self) -> &DatabaseDDL {
        match *self {
            #[cfg(feature = "postgres")]
            Platform::Postgres(ref pg) => pg,
            #[cfg(feature = "sqlite")]
            Platform::Sqlite(ref lite) => lite,
            #[cfg(feature = "mysql")]
            Platform::Mysql(ref my) => my,
        }
    }

    pub fn as_dev(&self) -> &DatabaseDev {
        match *self {
            #[cfg(feature = "postgres")]
            Platform::Postgres(ref pg) => pg,
            #[cfg(feature = "sqlite")]
            Platform::Sqlite(ref lite) => lite,
            #[cfg(feature = "mysql")]
            Platform::Mysql(ref my) => my,
        }
    }
}

impl Deref for Platform {
    type Target = Database;

    fn deref(&self) -> &Self::Target {
        debug!("using deref...");
        match *self {
            #[cfg(feature = "postgres")]
            Platform::Postgres(ref pg) => pg,
            #[cfg(feature = "sqlite")]
            Platform::Sqlite(ref lite) => lite,
            #[cfg(feature = "mysql")]
            Platform::Mysql(ref my) => my,
        }
    }
}


/// Postgres, Sqlite uses r2d2 connection manager,
/// Mysql has its own connection pooling
pub enum ManagedPool {
    #[cfg(feature = "postgres")]
    Postgres(Pool<PostgresConnectionManager>),
    #[cfg(feature = "sqlite")]
    Sqlite(Pool<SqliteConnectionManager>),
    #[cfg(feature = "mysql")]
    Mysql(Option<MyPool>),
}

impl ManagedPool {
    /// initialize the pool
    #[allow(unused)]
    pub fn init(url: &str, pool_size: usize) -> Result<Self, DbError> {
        let config = DbConfig::from_url(url);
        let pool_size = pool_size as u32;
        match config {
            Some(config) => {
                let platform: &str = &config.platform;
                match platform {
                    #[cfg(feature = "postgres")]
                    "postgres" => {
                        let manager = try!(PostgresConnectionManager::new(url, SslMode::None));
                        debug!("Creating a connection with a pool size of {}", pool_size);
                        let config = Config::builder().pool_size(pool_size).build();
                        let pool = try!(Pool::new(config, manager));
                        Ok(ManagedPool::Postgres(pool))
                    }

                    #[cfg(feature = "sqlite")]
                    "sqlite" => {
                        let manager = SqliteConnectionManager::new(&config.database);
                        let config = Config::builder().pool_size(pool_size).build();
                        let pool = try!(Pool::new(config, manager));
                        Ok(ManagedPool::Sqlite(pool))
                    }
                    #[cfg(feature = "mysql")]
                    "mysql" => {
                        let opts = MyOpts {
                            user: config.username,
                            pass: config.password,
                            db_name: Some(config.database),
                            tcp_addr: Some(config.host.unwrap().to_string()),
                            tcp_port: config.port.unwrap_or(3306),
                            ..Default::default()
                        };
                        let pool = try!(MyPool::new_manual(0, pool_size as usize, opts));
                        Ok(ManagedPool::Mysql(Some(pool)))
                    }

                    _ => unimplemented!(),
                }
            }
            None => {
                println!("Unable to parse url");
                Err(DbError::new("Error parsing url"))
            }
        }

    }

    /// a conection is created here
    pub fn connect(&self) -> Result<Platform, DbError> {
        match *self {
            #[cfg(feature = "postgres")]
            ManagedPool::Postgres(ref pool) => {
                match pool.get() {
                    Ok(conn) => {
                        let pg = Postgres::with_pooled_connection(conn);
                        Ok(Platform::Postgres(pg))
                    }
                    Err(e) => Err(DbError::new(&format!("Unable to connect due to {}", e))),
                }
            }
            #[cfg(feature = "sqlite")]
            ManagedPool::Sqlite(ref pool) => {
                match pool.get() {
                    Ok(conn) => {
                        let lite = Sqlite::with_pooled_connection(conn);
                        Ok(Platform::Sqlite(lite))
                    }
                    Err(e) => Err(DbError::new(&format!("Unable to connect due to {}", e))),
                }
            }
            #[cfg(feature = "mysql")]
            ManagedPool::Mysql(ref pool) => {
                let my = Mysql::with_pooled_connection(pool.clone().unwrap());// I hope cloning doesn't really clone the pool, just the Arc
                Ok(Platform::Mysql(my))
            }
        }
    }
}
