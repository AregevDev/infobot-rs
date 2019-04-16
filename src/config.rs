use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub token: String,
    pub prefix: String,
}

impl Config {
    pub fn from_toml<P: AsRef<Path>>(path: P) -> Self {
        toml::from_slice(fs::read(path).unwrap().as_slice()).unwrap()
    }
}
