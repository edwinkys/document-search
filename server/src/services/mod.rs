// TODO: Remove this once the service is implemented.
#![allow(dead_code)]

mod coordinator;
pub mod interface;

use crate::protos;
use crate::types::*;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;
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
            pool_size: 2,
        }
    }
}

#[derive(Debug)]
pub struct Service {
    config: Configuration,
    pool: PgPool,
    workers: Mutex<Vec<Worker>>,
}

impl Service {
    /// Creates an instance of the service with the given configuration.
    pub async fn new(config: &Configuration) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(config.pool_size as u32)
            .connect(config.database_url.as_str())
            .await
            .expect("Failed to connect to the database");

        Service {
            config: config.clone(),
            workers: Mutex::new(Vec::new()),
            pool,
        }
    }

    /// Adds a worker to the list of workers if it doesn't already exist.
    pub async fn add_worker(&self, worker: &Worker) {
        let mut workers = self.workers.lock().await;
        if workers.iter().any(|w| w.id == worker.id) {
            return;
        }

        workers.push(worker.clone());
    }

    /// Removes multiple workers from the list of workers by their IDs.
    pub async fn remove_workers(&self, ids: &[WorkerID]) {
        let mut workers = self.workers.lock().await;
        workers.retain(|worker| !ids.contains(&worker.id));
    }

    /// Returns a list of all workers.
    pub async fn workers(&self) -> Vec<Worker> {
        self.workers.lock().await.clone()
    }
}
