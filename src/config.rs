use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    cookie_secret: String,
    database_url: String,
    password_salt: String,
    port: u16,
    redis_url: String,
}

impl Config {
    pub fn cookie_secret(&self) -> &str {
        &self.cookie_secret
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    pub fn password_salt(&self) -> &str {
        &self.password_salt
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn redis_url(&self) -> &str {
        &self.redis_url
    }
}

pub fn load_config() -> Config {
    dotenv().ok();
    envy::from_env::<Config>().expect("Could not parse environment variables into config")
}
