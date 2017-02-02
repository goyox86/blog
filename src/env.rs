use std::str::FromStr;
use std::string::ToString;
use std::default::Default;
use std::fmt;

use rocket::config::Environment as RocketEnv;

#[derive(Clone, Debug, PartialEq)]
pub enum Env {
    Development,
    Test,
    Staging,
    Production
}

#[derive(Debug)]
pub enum ParseEnvError {
    UnknownEnv
}

impl fmt::Display for ParseEnvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseEnvError::UnknownEnv => write!(f, "Unknown environment")
        }
    }
}

impl FromStr for Env {
    type Err = ParseEnvError;

    fn from_str(s: &str) -> Result<Env, Self::Err>{
        match s {
            "development" => Ok(Env::Development),
            "test" => Ok(Env::Test),
            "staging" => Ok(Env::Staging),
            "production" => Ok(Env::Production),
            _ => Err(ParseEnvError::UnknownEnv)
        }
    }
}

impl Default for Env {
    fn default() -> Env {
        Env::Development
    }
}

impl ToString for Env {
    fn to_string(&self) -> String {
        match *self {
            Env::Development => String::from("development"),
            Env::Test => String::from("test"),
            Env::Staging => String::from("staging"),
            Env::Production => String::from("production"),
        }
    }
}


impl From<RocketEnv> for Env {
    fn from(env: RocketEnv) -> Env {
        match env {
             RocketEnv::Development => Env::Development,
             RocketEnv::Staging => Env::Staging,
             RocketEnv::Production => Env::Production
        }
    }
}

impl Env {
    fn is_development(&self) -> bool {
        *self == Env::Development
    }

    fn is_test(&self) -> bool {
        *self == Env::Test
    }

    fn is_staging(&self) -> bool {
        *self == Env::Staging
    }

    fn is_prod(&self) -> bool {
        *self == Env::Production
    }

    pub fn to_rocket(&self) -> RocketEnv {
        match self {
             &Env::Development => RocketEnv::Development,
             &Env::Staging => RocketEnv::Staging,
             &Env::Production => RocketEnv::Production,
             &Env::Test => RocketEnv::Development
        }
    }
}
