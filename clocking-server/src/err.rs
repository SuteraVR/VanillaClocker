use std::{io, string::FromUtf8Error};

use thiserror::Error;
use tokio_rustls::rustls;
use tracing_subscriber::util::TryInitError;

#[derive(Error, Debug)]
pub enum ClockerError {
    #[error("initialize tracing subscriber error: {0}")]
    InitializeTracingSubscriber(TryInitError),
    #[error("create tcp listener error: {0}, port: {1}")]
    CreateTCPListener(io::Error, u16),
    #[error("failed to accept new connection")]
    AcceptNewConnection(io::Error),
    #[error("failed to accept new stream")]
    AcceptNewStream(io::Error),
    #[error("private key pem section not found")]
    ReadPrivateKeyPEMSection,
    #[error("build tls server error: {0}")]
    BuildServer(rustls::Error),
    #[error("failed to load cert file: {0}")]
    LoadCertFile(io::Error),
    #[error("failed to load private key file: {0}")]
    LoadPrivateKeyFile(io::Error),
    #[error("failed to read buffer: {0}")]
    ReadBuffer(io::Error),
    #[error("failed to convert buffer to string: {0}")]
    ConvertBufferToString(FromUtf8Error),
}
