#![allow(unused, dead_code)]

use crate::core::config::AppConfig;
use human_panic::setup_panic;
use std::fs;

/// Represents a Dock application
///
/// A `App` instance is used to build and run a command line application from start to finish.
///
/// When a Dock app is initialized, by default, human_panic handler is set and on windows, an attempt to enable ansi support is made.
///
/// The suggested approach is to use the [`App::from_crate`] constructor to automatically build an application based on the crate config.
/// ```rsmno_run
///
/// fn main(){
///     App::from_crate()
///         .run()
/// }
/// ```
///
/// However, it can be set manually and built by each stage using their respective config setters.
///
/// ```rs,no_run
/// fn main(){
///     App::new()
///         .set_name("Dock")
///         .run()
/// }
/// ```
///  
/// The command line application starts when the [`App::run()`] method is called.
#[derive(Debug, Clone)]
pub struct App {
    pub config: AppConfig,
}

impl Default for App {
    fn default() -> Self {
        Self {
            config: AppConfig::new(),
        }
    }
}

impl App {
    /// The setup function called when a application is initialized.
    /// This performs all the preliminary setup required to make sure the Dock application sails smoothly.
    fn setup() {
        setup_panic!();

        #[cfg(windows)]
        let _ = ansi_term::enable_ansi_support();
    }

    /// Manual constructor
    ///
    /// All the config values should be set manually using the respective config setters
    pub fn new() -> Self {
        App::setup();

        Default::default()
    }

    /// Preferred constructor
    ///
    /// All the config values are automatically taken from the crate's `Config.toml` to build the application.
    pub fn from_crate() -> Self {
        App::setup();

        Self {
            config: AppConfig::from_crate(),
            ..Default::default()
        }
    }

    /// Property setter
    ///
    /// Sets the name of the application
    pub fn set_name(mut self, name: &str) -> Self {
        self.config.name = Some(name.to_string());

        self
    }

    /// Property setter
    ///
    /// Sets the description of the application
    pub fn set_description(mut self, description: &str) -> Self {
        self.config.description = Some(description.to_string());

        self
    }

    /// Property setter
    ///
    /// Sets the authors of the application
    pub fn set_authors(mut self, authors: Vec<String>) -> Self {
        self.config.authors = Some(authors);

        self
    }

    pub fn register_command(mut self) -> Self {
        todo!()
    }

    pub fn run(self) {
        todo!()
    }
}

#[cfg(test)]
mod app_tests {

    use super::*;

    #[test]
    fn manual_setup() {
        let app = App::new()
            .set_name("Dock-test")
            .set_description("Dock unit test application")
            .set_authors(vec!["Ferris".to_string()]);

        assert_eq!(app.config.name.as_ref().unwrap(), &"Dock-test".to_string());
        assert_eq!(
            app.config.description.as_ref().unwrap(),
            &"Dock unit test application".to_string()
        );
        assert_eq!(app.config.authors.as_ref().unwrap(), &vec!["Ferris".to_string()]);
    }

    #[test]
    fn missing_manual_setup() {
        let app = App::new()
            .set_name("Dock-test")
            .set_description("Dock unit test application");

        assert_eq!(app.config.name.as_ref().unwrap(), &"Dock-test".to_string());
        assert_eq!(
            app.config.description.as_ref().unwrap(),
            &"Dock unit test application".to_string()
        );
        assert_eq!(app.config.authors.as_ref(), None);
    }

    #[test]
    fn crate_config_setup() {
        let app = App::from_crate();

        assert_eq!(app.config.name.as_ref().unwrap(), &"dock".to_string());
        assert_eq!(
            app.config.description.as_ref().unwrap(),
            &"The simple, fast and powerful command line parser".to_string()
        );
        assert_eq!(app.config.authors.as_ref().unwrap(), &vec!["dimensionhq".to_string()]);
    }
}
