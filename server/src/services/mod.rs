// TODO: Remove this once the service is implemented.
#![allow(dead_code)]

mod coordinator;
pub mod interface;

use crate::protos;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use url::Url;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub database_url: Url,
    pub pool_size: u16,
}

#[cfg(test)]
impl Default for Configuration {
    fn default() -> Self {
        let database = "postgres://postgres:password@localhost:5432/postgres";
        Configuration {
            database_url: Url::parse(database).unwrap(),
            pool_size: 4,
        }
    }
}

#[derive(Debug)]
pub struct Service {
    config: Configuration,
    pool: PgPool,
}

impl Service {
    /// Creates an instance of the service with the given configuration.
    pub async fn new(config: &Configuration) -> Self {
        let config = config.clone();

        let pool = PgPoolOptions::new()
            .max_connections(config.pool_size as u32)
            .connect(config.database_url.as_str())
            .await
            .expect("Failed to connect to the database");

        Service { config, pool }
    }
}
