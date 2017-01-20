use toml::Parser;
use std::io::prelude::*;
use std::io::Error as IoError;
use std::fs::File;
use std::fmt;
use std::error;

use env::Env;

const CONFIG_DIR: &'static str = "./config";
const DB_CONFIG_FILE: &'static str = "database.toml";

#[derive(Debug)]
pub enum DbConfigError {
    Io(IoError),
    Parsing(String)
}

impl fmt::Display for DbConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DbConfigError::Io(_) => write!(f, "There was an error accessing the DB config file"),
            DbConfigError::Parsing(_) => write!(f, "There was an error parsing DB config file")
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

impl From<IoError> for DbConfigError {
    fn from(err: IoError) -> DbConfigError {
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
    pub port: i64,
}

impl DbConfig {
    pub fn load(env: &Env) -> Result<DbConfig, DbConfigError> {
        let config_file_path = format!("{}/{}", CONFIG_DIR, DB_CONFIG_FILE);
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
        let env_toml = toml.get(&env.to_string()).unwrap().as_table().unwrap();

        let adapter = match env_toml.get("adapter") {
            None => "postgres".to_string(),
            Some(adapter) => adapter.to_string()
        };
        let encoding = match env_toml.get("encoding") {
            None => "utf8".to_string(),
            Some(encoding) => adapter.to_string()
        };
        let database = match env_toml.get("database") {
            None => return Err(DbConfigError::Parsing(String::from("'database' key does not exist."))),
            Some(database) => database.to_string()
        };
        let username = match env_toml.get("username") {
            None => return Err(DbConfigError::Parsing(String::from("'username' key does not exist."))),
            Some(username) => username.to_string()
        };
        let password = match env_toml.get("password") {
            None => return Err(DbConfigError::Parsing(String::from("'password' key does not exist."))),
            Some(password) => password.to_string()
        };
        let host = match env_toml.get("host") {
            None => "localhost".to_string(),
            Some(host) => host.to_string()
        };
        let port = match env_toml.get("port") {
            None => 5432,
            Some(port) => port.as_integer().expect("invalid port")
        };

        Ok(DbConfig {
            adapter: adapter,
            encoding: encoding,
            database: database,
            username: username,
            password: password,
            host: host,
            port: port,
        })
    }

    pub fn url(&self) -> String {
        // FIXME: Get rid of that replace call :/
        format!("{}://{}:{}@{}:{}/{}", self.adapter, self.username, self.password, self.host, self.port, self.database).replace("\"", "")
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Db(DbConfigError)
}

impl From<DbConfigError> for ConfigError {
    fn from(err: DbConfigError) -> ConfigError {
        ConfigError::Db(err)
    }
}

#[derive(Debug)]
pub struct Config {
    database: DbConfig
}

impl Config {
    pub fn load(environment: &Env) -> Result<Config, ConfigError> {
        let database_config = DbConfig::load(environment)?;

        Ok(Config {
            database: database_config
        })
    }

    pub fn database(&self) -> &DbConfig {
       &self.database
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

