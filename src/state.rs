use redis::{aio::Connection as RedisConnection, Client as RedisClient};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::join;

use crate::env::{self, Environment};

pub struct AppState {
    db: Pool<Postgres>,
    env: Environment,
    redis: RedisConnection,
}

impl AppState {
    pub async fn new() -> AppState {
        let env = env::load_env();

        let redis = RedisClient::open(env.redis_url()).expect("Could not connect to Redis");
        let (db, redis) = join!(
            PgPoolOptions::new().connect(env.database_url()),
            redis.get_tokio_connection()
        );
        let db = db.expect("Could not connect to database");
        let redis = redis.expect("Could not connect to Redis");

        AppState { db, env, redis }
    }

    pub fn db(&self) -> &Pool<Postgres> {
        &self.db
    }

    pub fn env(&self) -> &Environment {
        &self.env
    }

    pub fn redis(&self) -> &RedisConnection {
        &self.redis
    }
}
