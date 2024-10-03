use std::{io, string::FromUtf8Error};

use thiserror::Error;
use tokio_rustls::rustls;

#[derive(Error, Debug)]
pub enum ClockerError {
    #[error("create tcp listener error. port: {0}")]
    CreateTCPListener(u16),
    #[error("accept new connection")]
    AcceptNewConnection,
    #[error("private key pem section not found")]
    PrivateKeyPEMSectionNotFound,
    #[error("unexpected io error. {0}")]
    UnexpectedIO(io::Error),
    #[error("unexpected rust ls error. {0}")]
    UnexpectedRustls(rustls::Error),
    #[error("unexpected convert error. {0}")]
    UnexpectedFromUtf(FromUtf8Error),
}
