mod config;

pub use config::{Config, config};

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
