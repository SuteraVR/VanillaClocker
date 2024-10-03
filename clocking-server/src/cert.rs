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
    let mut cert_file = match File::open(cert_path) {
        Ok(c) => c,
        Err(e) => return Err(ClockerError::UnexpectedIO(e).into()),
    };

    let cert = match rustls_pemfile::certs(&mut BufReader::new(&mut cert_file))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(c) => c,
        Err(e) => return Err(ClockerError::UnexpectedIO(e).into()),
    };

    Ok(cert)
}

#[instrument(skip_all, name = "read_private_key_file", level = "trace")]
pub fn read_private_key_file(
    private_key_path: String,
) -> Result<PrivateKeyDer<'static>, SpanErr<ClockerError>> {
    let mut private_key_file = match File::open(private_key_path) {
        Ok(c) => c,
        Err(e) => return Err(ClockerError::UnexpectedIO(e).into()),
    };

    let private_key = match rustls_pemfile::private_key(&mut BufReader::new(&mut private_key_file))
    {
        Ok(Some(c)) => c,
        Ok(None) => return Err(ClockerError::PrivateKeyPEMSectionNotFound.into()),
        Err(e) => return Err(ClockerError::UnexpectedIO(e).into()),
    };

    Ok(private_key)
}
