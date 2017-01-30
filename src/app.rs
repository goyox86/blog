use std::env as std_env;
use std::str::FromStr;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel;
use rocket::Rocket;

use config::Config;
use env::Env;
use models::*;
use endpoints;

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
        self.engine.unwrap().mount("/", routes![endpoints::index]).launch();
    }

    pub fn create_user(&self, username: &str, name: &str) -> User {
        use schema::users;

        let new_user = NewUser {
            username: username,
            name: name,
        };

        diesel::insert(&new_user)
            .into(users::table)
            .get_result::<User>(self.db_conn.as_ref().unwrap())
            .expect("Error saving new user")
    }

    pub fn create_post(&self, title: &str, body: &str, user: &User) {
        use schema::posts;

        let new_post = NewPost {
            title: title,
            body: body,
            user_id: Some(user.id),
        };

        diesel::insert(&new_post)
            .into(posts::table)
            .get_result::<Post>(self.db_conn.as_ref().unwrap())
            .expect("Error saving new post");
    }
}
