use redis::{aio::Connection as RedisConnection, Client as RedisClient};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::join;

use crate::config::{self, Config};

pub struct AppState {
    config: Config,
    db: Pool<Postgres>,
    redis: RedisConnection,
}

impl AppState {
    pub async fn new() -> AppState {
        let config = config::load_config();

        let redis = RedisClient::open(config.redis_url()).expect("Could not connect to Redis");
        let (db, redis) = join!(
            PgPoolOptions::new().connect(config.database_url()),
            redis.get_tokio_connection()
        );
        let db = db.expect("Could not connect to database");
        let redis = redis.expect("Could not connect to Redis");

        AppState { config, db, redis }
    }

    pub fn db(&self) -> &Pool<Postgres> {
        &self.db
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn redis(&self) -> &RedisConnection {
        &self.redis
    }
}
