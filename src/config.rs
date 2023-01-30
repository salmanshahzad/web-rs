use std::env;

use dotenv::dotenv;

pub struct Config {
    database_url: String,
    jwt_secret: String,
    password_salt: String,
    port: u16,
    redis_url: String,
}

impl Config {
    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    pub fn jwt_secret(&self) -> &str {
        &self.jwt_secret
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
    dotenv().expect("Could not load environment variables");

    let database_url = env::var("DATABASE_URL").expect("Invalid DATABASE_URL environment variable");
    let jwt_secret = env::var("JWT_SECRET").expect("Invalid JWT_SECRET environment variable");
    let password_salt =
        env::var("PASSWORD_SALT").expect("Invalid PASSWORD_SALT environment variable");
    let port = env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .expect("Invalid PORT environment variable");
    let redis_url = env::var("REDIS_URL").expect("Invalid REDIS_URL environment variable");

    Config {
        database_url,
        jwt_secret,
        password_salt,
        port,
        redis_url,
    }
}
