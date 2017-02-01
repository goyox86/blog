use std::env as std_env;
use std::str::FromStr;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel;
use rocket::Rocket;

use config::Config;
use env::Env;
use models::*;
use controllers::*;
use routes;

pub struct App {
    pub env: Env,
    pub config: Config,
    // TODO save here a diesel::Connection trait object
    pub db_conn: Option<PgConnection>,
    pub engine: Option<Rocket>
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
            db_conn: None,
            engine: None
        }
    }

    pub fn init(&mut self) {
        let db_url = self.config.db().url();
        self.db_conn = Some(PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url)));
        self.engine = Some(Rocket::ignite());
    }

    pub fn start(self) {
        let routes = routes::draw();
        self.engine.unwrap().mount("/", routes).launch();
    }
}
