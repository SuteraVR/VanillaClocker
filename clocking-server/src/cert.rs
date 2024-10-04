use crate::err::ClockerError;

use std::fs::File;
use std::io::BufReader;
use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};
use tracing::instrument;
use tracing_spanned::SpanErr;

#[instrument(skip_all, name = "read_cert_file", level = "trace")]
pub fn read_cert_file(
    cert_path: String,
) -> Result<Vec<CertificateDer<'static>>, SpanErr<ClockerError>> {
    let mut cert_file = File::open(cert_path).map_err(ClockerError::UnexpectedIO)?;

    let cert = rustls_pemfile::certs(&mut BufReader::new(&mut cert_file))
        .collect::<Result<Vec<_>, _>>()
        .map_err(ClockerError::UnexpectedIO)?;

    Ok(cert)
}

#[instrument(skip_all, name = "read_private_key_file", level = "trace")]
pub fn read_private_key_file(
    private_key_path: String,
) -> Result<PrivateKeyDer<'static>, SpanErr<ClockerError>> {
    let mut private_key_file = File::open(private_key_path).map_err(ClockerError::UnexpectedIO)?;

    let private_key = rustls_pemfile::private_key(&mut BufReader::new(&mut private_key_file))
        .map_err(ClockerError::UnexpectedIO)?
        .ok_or(ClockerError::PrivateKeyPEMSectionNotFound)?;

    Ok(private_key)
}
