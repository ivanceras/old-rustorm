use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use r2d2::Config;
use postgres::SslMode;
use config::DbConfig;
use database::{Database, DatabaseDDL, DatabaseDev};
use platform::Postgres;
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

#[cfg(feature = "sqlite")]
use r2d2_sqlite::SqliteConnectionManager;


/// the sql builder for each of the database platform
pub enum Platform {
    Postgres(Postgres),
    #[cfg(feature = "sqlite")]
    Sqlite(Sqlite),
    Oracle,
    #[cfg(feature = "mysql")]
    Mysql(Mysql),
}

impl Platform {
    pub fn as_ref(&self) -> &Database {
        match *self {
            Platform::Postgres(ref pg) => pg,
            #[cfg(feature = "sqlite")]
            Platform::Sqlite(ref lite) => lite,
            #[cfg(feature = "mysql")]
            Platform::Mysql(ref my) => my,
            _ => unimplemented!(),
        }
    }

    pub fn as_ddl(&self) -> &DatabaseDDL {
        match *self {
            Platform::Postgres(ref pg) => pg,
            #[cfg(feature = "sqlite")]
            Platform::Sqlite(ref lite) => lite,
            #[cfg(feature = "mysql")]
            Platform::Mysql(ref my) => my,
            _ => unimplemented!(),
        }
    }

    pub fn as_dev(&self) -> &DatabaseDev {
        match *self {
            Platform::Postgres(ref pg) => pg,
            #[cfg(feature = "sqlite")]
            Platform::Sqlite(ref lite) => lite,
            #[cfg(feature = "mysql")]
            Platform::Mysql(ref my) => my,
            _ => unimplemented!(),
        }
    }
}

impl Deref for Platform{
	type Target = Database;

	fn deref(&self)->&Self::Target{
		debug!("using deref...");
        match *self {
            Platform::Postgres(ref pg) => pg,
            #[cfg(feature = "sqlite")]
            Platform::Sqlite(ref lite) => lite,
            #[cfg(feature = "mysql")]
            Platform::Mysql(ref my) => my,
            _ => unimplemented!(),
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
    #[cfg(feature = "mysql")]
    Mysql(Option<MyPool>),
}

impl ManagedPool {
    /// initialize the pool
    pub fn init(url: &str, pool_size: usize) -> Result<Self, DbError> {
        let config = DbConfig::from_url(url);
        match config {
            Some(config) => {
                let platform: &str = &config.platform;
                match platform {
                    "postgres" => {
                        let manager = try!(PostgresConnectionManager::new(url, SslMode::None));
                        debug!("Creating a connection with a pool size of {}", pool_size);
                        let config = Config::builder().pool_size(pool_size as u32).build();
                        let pool = try!(Pool::new(config, manager));
                        Ok(ManagedPool::Postgres(pool))
                    }

                    #[cfg(feature = "sqlite")]
                    "sqlite" => {
                        let manager = try!(SqliteConnectionManager::new(&config.database));
                        let config = Config::builder().pool_size(pool_size as u32).build();
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
                        let pool = try!(MyPool::new_manual(0, pool_size, opts));
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
            #[cfg(feature = "mysql")]
            ManagedPool::Mysql(ref pool) => {
                let my = Mysql::with_pooled_connection(pool.clone().unwrap());// I hope cloning doesn't really clone the pool, just the Arc
                Ok(Platform::Mysql(my))
            }
            _ => Err(DbError::new("Any other database is not yet supported")),
        }
    }
}
