use std::env as std_env;
use std::str::FromStr;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use rocket::Rocket;
use rocket::config::Config as RocketConfig;

use config::Config;
use env::Env;
use controllers::*;
use routes;

const CONFIG_DIR: &'static str = "./config";
const DB_CONFIG_FILE: &'static str = "database.toml";
const ROCKET_CONFIG_FILE: &'static str = "Rocket.toml";

pub struct App {
    pub env: Env,
    pub config: Config,
    // TODO save here a diesel::Connection trait object
    pub db_conn: Option<PgConnection>,
    pub rocket: Option<Rocket>
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
            rocket: None
        }
    }

    pub fn init(&mut self) {
        let db_url = self.config.db().url();
        self.db_conn = Some(PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url)));
        let rocket_config = Config::load_rocket(&self.env).unwrap();
        self.rocket = Some(Rocket::custom(rocket_config, true));
    }

    pub fn start(self) {
        let routes = routes::draw();
        self.rocket.unwrap().mount("/", routes).launch();
    }
}
