// TODO: Remove this once the service is implemented.
#![allow(dead_code)]

mod coordinator;
pub mod interface;

use crate::protos;
use serde::Serialize;
use std::sync::Arc;
use tonic::{Request, Response, Status};

#[derive(Debug, Clone)]
pub struct Configuration {}

#[derive(Debug)]
pub struct Service {
    config: Configuration,
}

impl Service {
    /// Creates an instance of the service with the given configuration.
    pub fn new(config: &Configuration) -> Self {
        Service {
            config: config.clone(),
        }
    }
}
