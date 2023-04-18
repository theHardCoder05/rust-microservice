use std::sync::Mutex;

use crate::{users::Users};

use tonic::{Request, Response, Status};

pub mod authentication {
    tonic::include_proto!("authentication");
}

pub use tonic::transport::Server;

pub struct 