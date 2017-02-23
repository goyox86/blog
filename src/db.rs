use std::fmt;
use std::error;

use diesel::pg::PgConnection;
use r2d2::Config;
use r2d2::{Pool, InitializationError};
use r2d2_diesel::ConnectionManager;

use config::DbConfig;

use diesel::result::Error as DieselError;
use r2d2::GetTimeout;

#[derive(Debug)]
pub enum DbError {
    Db(DieselError),
    PoolInitialization(InitializationError),
    PoolTimeout(GetTimeout),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DbError::Db(_) => write!(f, "Db error"),
            DbError::PoolInitialization(_) => write!(f, "Db pool could not be initialized"),
            DbError::PoolTimeout(_) => write!(f, "Timeout while trying to access the Db Pool"),
        }
    }
}

impl error::Error for DbError {
    fn description(&self) -> &str {
        match *self {
            DbError::Db(ref err) => err.description(),
            DbError::PoolInitialization(ref err) => err.description(),
            DbError::PoolTimeout(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            DbError::Db(ref err) => Some(err),
            DbError::PoolInitialization(ref err) => Some(err),
            DbError::PoolTimeout(ref err) => Some(err),
        }
    }
}

impl From<DieselError> for DbError {
    fn from(err: DieselError) -> DbError {
        DbError::Db(err)
    }
}

impl From<InitializationError> for DbError {
    fn from(err: InitializationError) -> DbError {
        DbError::PoolInitialization(err)
    }
}

impl From<GetTimeout> for DbError {
    fn from(err: GetTimeout) -> DbError {
        DbError::PoolTimeout(err)
    }
}

pub struct Db {
    pub pool: Option<Pool<ConnectionManager<PgConnection>>>,
    pub config: DbConfig,
}

type DbPool = Pool<ConnectionManager<PgConnection>>;

impl Db {
    pub fn new(config: DbConfig) -> Db {
        Db {
            pool: None,
            config: config,
        }
    }

    pub fn init(&mut self) -> Result<(), DbError> {
        let db_url = self.config.url();
        let config = Config::default();
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool = Pool::new(config, manager)?;
        self.pool = Some(pool);
        Ok(())
    }

    pub fn pool(&self) -> &DbPool {
        self.pool.as_ref().expect("Db Pool not available. Maybe call 'init()' first?")
    }
}
