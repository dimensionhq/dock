use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct CargoConfig {
    package: CargoConfigPackage,
}

#[derive(Serialize, Deserialize)]
pub struct CargoConfigPackage {
    name: Option<String>,
    description: Option<String>,
    authors: Option<Vec<String>>,
}

///Holds the application config
///
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub name: Option<String>,
    pub description: Option<String>,
    pub authors: Option<Vec<String>>,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
            authors: None,
        }
    }

    pub fn from_crate() -> Self {
        let config = toml::from_str::<CargoConfig>(
            fs::read_to_string("./Cargo.toml")
                .expect("Failed to read Cargo.toml")
                .as_str(),
        )
        .expect("Failed to parse Cargo.toml");

        Self {
            name: config.package.name,
            description: config.package.description,
            authors: config.package.authors,
        }
    }
}
