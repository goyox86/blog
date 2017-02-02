use toml::Parser;
use rocket::config::ConfigError as RocketConfigError;
use rocket::config::Config as RocketConfig;
use rocket::config::Environment as RocketEnv;

use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::fmt;
use std::error;
use std::path::Path;

use env::Env;

const CONFIG_DIR: &'static str = "./config";
const DB_CONFIG_FILE: &'static str = "database.toml";
const ROCKET_CONFIG_FILE: &'static str = "Rocket.toml";

#[derive(Debug)]
pub enum DbConfigError {
    Io(io::Error),
    Parsing(String)
}

impl fmt::Display for DbConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DbConfigError::Io(_) => write!(f, "Error accessing the DB config file"),
            DbConfigError::Parsing(_) => write!(f, "Error parsing DB config file")
        }
    }
}

impl error::Error for DbConfigError {
    fn description(&self) -> &str {
        match *self {
            DbConfigError::Io(ref err) => err.description(),
            DbConfigError::Parsing(ref err) => &err
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            DbConfigError::Io(ref err) => Some(err),
            DbConfigError::Parsing(_) => Some(self)
        }
    }
}

impl From<io::Error> for DbConfigError {
    fn from(err: io::Error) -> DbConfigError {
        DbConfigError::Io(err)
    }
}

#[derive(Debug)]
pub struct DbConfig {
    pub adapter: String,
    pub encoding: String,
    pub database: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16
}

impl DbConfig {
    pub fn new(adapter: &str, encoding: &str, database: &str,
               username: &str, password: &str, host: &str,
               port: u16) -> DbConfig {
        DbConfig {
            adapter: adapter.to_owned(),
            encoding: encoding.to_owned(),
            database: database.to_owned(),
            username: username.to_owned(),
            password: password.to_owned(),
            host: host.to_owned(),
            port: port
        }
    }

    pub fn load(env: &Env) -> Result<DbConfig, DbConfigError> {
        let config_file_path = Path::new(CONFIG_DIR).join(DB_CONFIG_FILE);
        let mut config_file = File::open(config_file_path)?;
        let mut buffer = String::new();
        config_file.read_to_string(&mut buffer)?;

        let mut parser = Parser::new(&buffer);
        let toml = match parser.parse() {
            None => {
                let desc = parser.errors.iter().fold(String::new(), |acc, ref error| acc + &format!("{}", error));
                return Err(DbConfigError::Parsing(format!("Parsing error {}", desc)));
            },
            Some(toml) => toml
        };

        let env_toml = match toml.get(&env.to_string()) {
           None => return Err(DbConfigError::Parsing(format!("no configuration found for env '{}'", env.to_string()))),
            Some(toml) => match toml.as_table() {
                None => return Err(DbConfigError::Parsing(format!("configuration section for env '{}' does not have the correct format", env.to_string()))),
                Some(toml) => toml
            }
        };

        let adapter = match env_toml.get("adapter") {
            None => "postgres",
            Some(adapter) => adapter.as_str().expect("invalid adapter: must me a string")
        };

        let encoding = match env_toml.get("encoding") {
            None => "utf8",
            Some(encoding) => encoding.as_str().expect("invalid encoding: must me a string")
        };

        let database = match env_toml.get("database") {
            None => return Err(DbConfigError::Parsing(String::from("'database' key does not exist."))),
            Some(database) => database.as_str().expect("invalid database: must me a string")
        };

        let username = match env_toml.get("username") {
            None => return Err(DbConfigError::Parsing(String::from("'username' key does not exist."))),
            Some(username) => username.as_str().expect("invalid username: must me a string")
        };

        let password = match env_toml.get("password") {
            None => return Err(DbConfigError::Parsing(String::from("'password' key does not exist."))),
            Some(password) => password.as_str().expect("invalid password: must me a string")
        };

        let host = match env_toml.get("host") {
            None => "localhost",
            Some(host) => host.as_str().expect("invalid host: must me a string")
        };

        let port = match env_toml.get("port") {
            None => 5432,
            Some(port) => port.as_integer().expect("invalid port: must be an integer")
        };

        Ok(Self::new(adapter, encoding, database, username, password, host, port as u16))
    }

    pub fn url(&self) -> String {
        format!("{}://{}:{}@{}:{}/{}", self.adapter, self.username, self.password, self.host, self.port, self.database)
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Db(DbConfigError),
    Rocket(RocketConfigError)
}

impl From<DbConfigError> for ConfigError {
    fn from(err: DbConfigError) -> ConfigError {
        ConfigError::Db(err)
    }
}

impl From<RocketConfigError> for ConfigError {
    fn from(err: RocketConfigError) -> ConfigError {
        ConfigError::Rocket(err)
    }
}

#[derive(Debug)]
pub struct Config {
    db: DbConfig,
    rocket: RocketConfig
}

impl Config {
    pub fn load(env: &Env) -> Result<Config, ConfigError> {
        let database_config = DbConfig::load(env)?;

        Ok(Config {
            db: database_config,
            rocket: RocketConfig::default_for(env.to_rocket(), &format!("{}/{}", CONFIG_DIR, ROCKET_CONFIG_FILE))?
        })
    }

    pub fn db(&self) -> &DbConfig {
       &self.db
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

