use std::{io, string::FromUtf8Error};

use thiserror::Error;
use tokio_rustls::rustls;
use tracing_subscriber::util::TryInitError;

#[derive(Error, Debug)]
pub enum ClockerError {
    #[error("initialize tracing subscriber error. {0}")]
    InitializeTracingSubscriber(TryInitError),
    #[error("create tcp listener error. err: {0}, port: {1}")]
    CreateTCPListener(io::Error, u16),
    #[error("accept new connection")]
    AcceptNewConnection(io::Error),
    #[error("private key pem section not found")]
    PrivateKeyPEMSectionNotFound,
    #[error("unexpected io error. {0}")]
    UnexpectedIO(io::Error),
    #[error("unexpected rust ls error. {0}")]
    UnexpectedRustls(rustls::Error),
    #[error("unexpected convert error. {0}")]
    UnexpectedFromUtf(FromUtf8Error),
}
