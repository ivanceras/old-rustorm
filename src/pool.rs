use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use r2d2::Config;
use postgres::SslMode;
use config::DbConfig;
use database::{Database, DatabaseDDL, DatabaseDev};
use platform::Postgres;
#[cfg(feature = "sqlite")]
use platform::Sqlite;
use platform::Mysql;
use mysql::conn::pool::MyPool;
use mysql::conn::MyOpts;
use database::DbError;

#[cfg(feature = "sqlite")]
use r2d2_sqlite::SqliteConnectionManager;


/// the sql builder for each of the database platform
pub enum Platform {
    Postgres(Postgres),
    #[cfg(feature = "sqlite")]
    Sqlite(Sqlite),
    Oracle,
    Mysql(Mysql),
}


impl Platform{

    pub fn as_ref(&self) -> &Database {
        match *self {
            Platform::Postgres(ref pg) => pg,
            #[cfg(feature = "sqlite")]
            Platform::Sqlite(ref lite) => lite,
            Platform::Mysql(ref my) => my,
            _ => panic!("others not yet.."),
        }
    }
    pub fn as_ddl(&self) -> &DatabaseDDL {
        match *self {
            Platform::Postgres(ref pg) => pg,
            #[cfg(feature = "sqlite")]
            Platform::Sqlite(ref lite) => lite,
            Platform::Mysql(ref my) => my,
            _ => panic!("others not yet.."),
        }
    }

    pub fn as_dev(&self) -> &DatabaseDev {
        match *self {
            Platform::Postgres(ref pg) => pg,
            #[cfg(feature = "sqlite")]
            Platform::Sqlite(ref lite) => lite,
            _ => panic!("others not yet.."),
        }
    }
}

/// Postgres, Sqlite uses r2d2 connection manager,
/// Mysql has its own connection pooling
pub enum ManagedPool {
    Postgres(Pool<PostgresConnectionManager>),
    #[cfg(feature = "sqlite")]
    Sqlite(Pool<SqliteConnectionManager>),
    Oracle,
    Mysql(Option<MyPool>),
}

impl ManagedPool{

    /// initialize the pool
    pub fn init(url: &str, pool_size: usize) -> Result<Self, DbError> {
        let config = DbConfig::from_url(url);
        match config {
            Some(config) => {
                let platform: &str = &config.platform;
                match platform {
                    "postgres" => {
                        let manager = PostgresConnectionManager::new(url, SslMode::None).unwrap();
                        println!("Creating a connection with a pool size of {}", pool_size);
                        let config = Config::builder().pool_size(pool_size as u32).build();
                        let pool = Pool::new(config, manager);
                        match pool {
                            Ok(pool) => Ok(ManagedPool::Postgres(pool)),
                            Err(e) => {
                                println!("Unable to create a pool");
                                Err(DbError::new(&format!("{}", e)))
                            }
                        }

                    }
                    #[cfg(feature = "sqlite")]
                    "sqlite" => {
                        let manager = SqliteConnectionManager::new(&config.database).unwrap();
                        let config = Config::builder().pool_size(pool_size as u32).build();
                        match Pool::new(config, manager) {
                            Ok(pool) => Ok(ManagedPool::Sqlite(pool)),
                            Err(e) => {
                                println!("Unable to create a pool");
                                Err(DbError::new(&format!("{}", e)))
                            }
                        }

                    }
                    "mysql" => {
                        let opts = MyOpts {
                            user: config.username,
                            pass: config.password,
                            db_name: Some(config.database),
                            tcp_addr: Some(config.host.unwrap().to_string()),
                            tcp_port: config.port.unwrap_or(3306),
                            ..Default::default()
                        };
                        match MyPool::new_manual(0, pool_size, opts) {
                            Ok(pool) => Ok(ManagedPool::Mysql(Some(pool))),
                            Err(e) => {
                                println!("Unable to create a pool");
                                Err(DbError::new(&format!("{}", e)))
                            }
                        }

                    }
                    _ => panic!("not yet"),
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
            ManagedPool::Postgres(ref pool) => {
                match pool.get() {
                    Ok(conn) => {
                        let pg = Postgres::with_pooled_connection(conn);
                        Ok(Platform::Postgres(pg))
                    }
                    Err(e) => {
                        Err(DbError::new(&format!("Unable to connect due to {}", e)))
                    }
                }
            }
            #[cfg(feature = "sqlite")]
            ManagedPool::Sqlite(ref pool) => {
                match pool.get() {
                    Ok(conn) => {
                        let lite = Sqlite::with_pooled_connection(conn);
                        Ok(Platform::Sqlite(lite))
                    }
                    Err(e) => {
                        Err(DbError::new(&format!("Unable to connect due to {}", e)))
                    }
                }
            }
            ManagedPool::Mysql(ref pool) => {
                let my = Mysql::with_pooled_connection(pool.clone().unwrap());// I hope cloning doesn't really clone the pool, just the Arc
                Ok(Platform::Mysql(my))
            }
            _ => Err(DbError::new("Any other database is not yet supported")),
        }
    }
}
