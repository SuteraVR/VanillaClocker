use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClockerError {
    #[error("create tcp listener error. port: {0}")]
    CreateTCPListener(u16),
    #[error("accept new connection")]
    AcceptNewConnection,
    #[error("unexpected io error. {0}")]
    UnexpectedIO(io::Error),
    #[error("unexpected error")]
    Unexpected,
}
