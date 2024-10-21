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
    #[error("accept new stream")]
    AcceptNewStream(io::Error),
    #[error("private key pem section not found")]
    ReadPrivateKeyPEMSection,
    #[error("unexpected rust ls error. {0}")]
    BuildServer(rustls::Error),
    #[error("failed to open file. {0}")]
    OpenFile(io::Error),
    #[error("failed to read file. {0}")]
    ReadFile(io::Error),
    #[error("failed to read buffer. {0}")]
    ReadBuffer(io::Error),
    #[error("failed to convert string. {0}")]
    ConvertBuffer(FromUtf8Error),
}
