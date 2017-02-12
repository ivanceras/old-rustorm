#[cfg(feature = "postgres")]
pub mod postgres;
#[cfg(feature = "sqlite")]
pub mod sqlite;
#[cfg(feature = "mysql")]
pub mod mysql;

#[cfg(feature = "postgres")]
pub use self::postgres::Postgres;
#[cfg(feature = "sqlite")]
pub use self::sqlite::Sqlite;
#[cfg(feature = "mysql")]
pub use self::mysql::Mysql;

use std::error::Error;
use std::fmt;
#[cfg(feature = "postgres")]
use postgres::error::Error as PgError;
#[cfg(feature = "mysql")]
use mysql::error::MyError;
#[cfg(feature = "sqlite")]
use rusqlite::SqliteError;

#[derive(Debug)]
pub enum PlatformError {
    #[cfg(feature = "postgres")]
    PostgresError(PgError),
    #[cfg(feature = "mysql")]
    MySQLError(MyError),
    #[cfg(feature = "sqlite")]
    SqliteError(SqliteError),
}

impl Error for PlatformError {
    fn description(&self) -> &str {
        match *self {
            #[cfg(feature = "postgres")]
            PlatformError::PostgresError(ref err) => err.description(),
            #[cfg(feature = "mysql")]
            PlatformError::MySQLError(ref err) => err.description(),
            #[cfg(feature = "sqlite")]
            PlatformError::SqliteError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            #[cfg(feature = "postgres")]
            PlatformError::PostgresError(ref err) => Some(err),
            #[cfg(feature = "mysql")]
            PlatformError::MySQLError(ref err) => Some(err),
            #[cfg(feature = "sqlite")]
            PlatformError::SqliteError(ref err) => Some(err),
        }
    }
}

impl fmt::Display for PlatformError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            #[cfg(feature = "postgres")]
            PlatformError::PostgresError(ref err) => write!(f, "PostgreSQL error: {}", err),
            #[cfg(feature = "mysql")]
            PlatformError::MySQLError(ref err) => write!(f, "MySQL error: {}", err),
            #[cfg(feature = "sqlite")]
            PlatformError::SqliteError(ref err) => write!(f, "SQlite error: {}", err),
        }
    }
}

#[cfg(feature = "postgres")]
impl From<PgError> for PlatformError {
    fn from(err: PgError) -> Self {
        PlatformError::PostgresError(err)
    }
}


#[cfg(feature = "mysql")]
impl From<MyError> for PlatformError {
    fn from(err: MyError) -> Self {
        PlatformError::MySQLError(err)
    }
}

#[cfg(feature = "sqlite")]
impl From<SqliteError> for PlatformError {
    fn from(err: SqliteError) -> Self {
        PlatformError::SqliteError(err)
    }
}
