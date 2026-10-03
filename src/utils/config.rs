use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    app_name: String,
    default_path: String,
    pub db_conn_str: String,
}

// methode
impl ::std::default::Default for AppConfig {
    fn default() -> Self {
        let app_name: &'static str = env!("CARGO_PKG_NAME");
        Self {
            app_name: app_name.to_string(),
            default_path: "makedeveasy/rust-cli".into(),
            db_conn_str: String::new(),
        }
    }
}

pub trait Builder {
    fn new() -> Self;
    fn set_config_path(&mut self, config_path: &str) -> &mut Self;
    fn set_db_conn_str(&mut self, conn_str: &str) -> &mut Self;
    fn build(&self) -> AppConfig;
}

impl Builder for AppConfig {
    fn new() -> Self {
        let app_name: &'static str = env!("CARGO_PKG_NAME");
        AppConfig {
            app_name: app_name.to_string(),
            default_path: "".into(),
            db_conn_str: String::new(),
        }
    }

    fn set_config_path(&mut self, config_path: &str) -> &mut Self {
        self.default_path = config_path.into();
        self
    }

    fn set_db_conn_str(&mut self, conn_str: &str) -> &mut Self {
        self.db_conn_str = conn_str.to_string();
        self
    }

    fn build(&self) -> AppConfig {
        AppConfig {
            app_name: self.app_name.clone(),
            default_path: self.default_path.clone(),
            db_conn_str: self.db_conn_str.clone(),
        }
    }
}