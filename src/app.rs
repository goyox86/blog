use std::env as std_env;
use std::str::FromStr;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use r2d2::Config as R2D2Config;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

use config::Config;
use env::Env;
use models::*;

pub struct App {
    pub env: Env,
    pub config: Config,
    pub db: Option<Pool<ConnectionManager<PgConnection>>>
}

impl App {
    pub fn new() -> App {
        let env_str = &std_env::var("BLOG_ENV").unwrap_or(format!("development"));
        let env = Env::from_str(env_str).unwrap();
        let config = Config::load(&env).expect("Error loading config!");
        let db_url = config.db().url();

        App {
            env: env,
            config: config,
            db: None
        }
    }

    pub fn start(&mut self) {
        let db_url = self.config.db().url();
        let config = R2D2Config::default();
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool = Pool::new(config, manager).expect("Failed to create DB connection pool.");
        self.db = Some(pool);
    }

    pub fn db(&self) -> &Pool<ConnectionManager<PgConnection>> {
       self.db.as_ref().unwrap()
    }
}
