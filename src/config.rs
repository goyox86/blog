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
pub enum DatabaseError {
    Io(IoError)
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DatabaseError::Io(_) => write!(f, "There was an error reading DB config file")
        }
    }
}

impl error::Error for DatabaseError {
    fn description(&self) -> &str {
        match *self {
            DatabaseError::Io(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            DatabaseError::Io(ref err) => Some(err)
        }
    }
}

impl From<IoError> for DatabaseError {
    fn from(err: IoError) -> DatabaseError {
        DatabaseError::Io(err)
    }
}

#[derive(Debug)]
pub struct DatabaseConfig {
    pub adapter: String,
    pub encoding: String,
    pub database: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: i64,
    pub pool: i64
}

//TODO: Shorten all of this to 'Db' instead of 'Database'
impl DatabaseConfig {
    pub fn load(env: &Env) -> Result<DatabaseConfig, DatabaseError> {
        let config_file_path = format!("{}/{}", CONFIG_DIR, DB_CONFIG_FILE);
        let mut config_file = File::open(config_file_path)?;
        let mut buffer = String::new();
        config_file.read_to_string(&mut buffer)?;

        let toml = Parser::new(&buffer).parse().unwrap();
        let env_toml = toml.get(&env.to_string()).unwrap().as_table().unwrap();

        Ok(DatabaseConfig {
            adapter: env_toml.get("adapter").unwrap().to_string(),
            encoding: env_toml.get("encoding").unwrap().to_string(),
            database: env_toml.get("database").unwrap().to_string(),
            username: env_toml.get("username").unwrap().to_string(),
            password: env_toml.get("password").unwrap().to_string(),
            host: env_toml.get("host").unwrap().to_string(),
            port: env_toml.get("port").unwrap().as_integer().unwrap(),
            pool: env_toml.get("pool").unwrap().as_integer().unwrap(),
        })
    }

    pub fn url(&self) -> String {
        // FIXME: Get rid of that replace call :/
        format!("{}://{}:{}@{}:{}/{}", self.adapter, self.username, self.password, self.host, self.port, self.database).replace("\"", "")
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Db(DatabaseError)
}

#[derive(Debug)]
pub struct Config {
    database: DatabaseConfig
}

impl Config {
    pub fn load(environment: &Env) -> Result<Config, ConfigError> {
        let database_config = DatabaseConfig::load(environment).unwrap();

        Ok(Config {
            database: database_config
        })
    }

    pub fn database(&self) -> &DatabaseConfig {
       &self.database
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

