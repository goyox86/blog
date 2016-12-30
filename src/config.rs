use toml::Parser;
use std::io::prelude::*;
use std::io::Error as IoError;
use std::fs::File;
use std::fmt;
use std::error;

const DEFAULT_FILE_NAME: &'static str = "database.toml";

#[derive(Debug)]
pub enum DatabaseError {
    Io(IoError)
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DatabaseError::Io(_) => write!(f, "There was an error loading DB config file")
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

impl DatabaseConfig {
    pub fn load() -> Result<DatabaseConfig, DatabaseError> {
        let config_file_path = format!("./config/{}", DEFAULT_FILE_NAME);
        let mut config_file = File::open(config_file_path)?;
        let mut buffer = String::new();
        config_file.read_to_string(&mut buffer)?;

        let toml = Parser::new(&buffer).parse().unwrap();
        let t = toml.get("development").unwrap().as_table().unwrap();

        Ok(DatabaseConfig {
            adapter: t.get("adapter").unwrap().to_string(),
            encoding: t.get("encoding").unwrap().to_string(),
            database: t.get("database").unwrap().to_string(),
            username: t.get("username").unwrap().to_string(),
            password: t.get("password").unwrap().to_string(),
            host: t.get("host").unwrap().to_string(),
            port: t.get("port").unwrap().as_integer().unwrap(),
            pool: t.get("pool").unwrap().as_integer().unwrap(),
        })
    }

    pub fn url(&self) -> String {
        // FIXME: Get rid of that replace call :/
        format!("{}://{}:{}@{}:{}/{}", self.adapter, self.username, self.password, self.host, self.port, self.database).replace("\"", "")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

}

