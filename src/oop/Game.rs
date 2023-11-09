use std::fs;

use serde_json::{Error as SerdeError, Value};

use super::Config::Config;
use super::Interface::Interface;

//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
#[derive(Debug)]
pub enum ConfigErrors {
    FsErrorOwn(std::io::Error),
    SerdeErrorOwn(SerdeError),
}

//-----------------------------------------------------------------------------
impl ConfigErrors {
    pub fn kind(&self) -> ConfigErrorKind {
        match self {
            ConfigErrors::FsErrorOwn(_) => ConfigErrorKind::FsError,
            ConfigErrors::SerdeErrorOwn(_) => ConfigErrorKind::SerdeError,
        }
    }
}

//-----------------------------------------------------------------------------
impl From<SerdeError> for ConfigErrors {
    fn from(error: SerdeError) -> Self {
        ConfigErrors::SerdeErrorOwn(error)
    }
}

//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
#[derive(Debug)]
pub enum ConfigErrorKind {
    FsError,
    SerdeError,
}

//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
pub struct Game {
    io: Interface,
    pub config: serde_json::Value,
}

//-----------------------------------------------------------------------------
impl Game {
    pub fn new(io: Interface) -> Self {
        Game {
            io,
            config: Value::Null,
        }
    }

    pub fn load_config(&mut self, file_name: &str) -> Result<(), ConfigErrors> {
        let tmp_config = match fs::read_to_string(file_name) {
            Ok(file) => file,
            Err(error) => return Err(ConfigErrors::FsErrorOwn(error)),
        };

        self.config = serde_json::from_str(&tmp_config)?;

        Config::validate_config(&self.config);

        Ok(())
    }

    pub fn print_config(&self) {
        println!("{}", serde_json::to_string_pretty(&self.config).unwrap());
    }
}
