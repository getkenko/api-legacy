use std::fs;

use once_cell::sync::Lazy;
use serde::Deserialize;

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::new("config.toml"));

#[derive(Deserialize)]
pub struct Config {
    pub cdn_url: String,
    pub default_avatar: String,
    pub user_check_interval: i64,

    pub bind: BindConfig,
    pub cache: CacheConfig,
    pub jwt: JwtConfig,
    pub rate_limit: RateLimitConfig,
}

impl Config {
    pub fn new(path: &str) -> Self {
        let content = fs::read_to_string(path).unwrap();
        let parsed = toml::from_str(&content).unwrap();
        parsed
    }
}

#[derive(Deserialize)]
pub struct BindConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct CacheConfig {
    pub redis_url: String,
}

#[derive(Deserialize)]
pub struct JwtConfig {
    pub duration: i64,
}

#[derive(Deserialize)]
pub struct RateLimitConfig {
    pub max_requests: u32,
    pub reset_after: i64,
}