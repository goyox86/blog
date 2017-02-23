
use diesel::pg::PgConnection;
use r2d2::Config;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

use config::DbConfig;

pub struct Db {
    pub pool: Option<Pool<ConnectionManager<PgConnection>>>,
    pub config: DbConfig,
}

impl Db {
    pub fn new(config: DbConfig) -> Db {
        Db {
            pool: None,
            config: config,
        }
    }

    pub fn init(&mut self) {
        let db_url = self.config.url();
        let config = Config::default();
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool = Pool::new(config, manager).expect("Failed to create DB connection pool.");
        self.pool = Some(pool);
    }

    pub fn pool(&self) -> &Pool<ConnectionManager<PgConnection>> {
        self.pool.as_ref().unwrap()
    }
}
