mod err;

use clap::Parser;
use dotenvy::dotenv;
use err::ClockerError;
use std::fs::File;
use std::io::BufReader;
use std::net::Ipv4Addr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::{rustls, TlsAcceptor};
use tracing::instrument;
use tracing_error::ErrorLayer;
use tracing_spanned::SpanErr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(short = 'p', long = "port", env, default_value = "3000")]
    port: u16,
    #[clap(short = 'c', long = "cert", env)]
    cert_path: String,
    #[clap(short = 'k', long = "private", env)]
    private_key_path: String,
}

#[tokio::main]
#[instrument(skip_all, name = "main", level = "trace")]
async fn main() -> Result<(), SpanErr<ClockerError>> {
    tracing_subscriber::Registry::default()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_filter(tracing_subscriber::filter::LevelFilter::INFO),
        )
        .with(ErrorLayer::default())
        .try_init()
        .expect("failed to initialize subscriber");

    let _ = dotenv();

    let args = Args::parse();

    let Ok(listener) = TcpListener::bind((Ipv4Addr::UNSPECIFIED, args.port)).await else {
        return Err(ClockerError::CreateTCPListener(args.port).into());
    };

    let acceptor = get_tls_acceptor(args.cert_path, args.private_key_path)?;

    loop {
        let Ok((stream, _)) = listener.accept().await else {
            return Err(ClockerError::AcceptNewConnection.into());
        };

        let acceptor = acceptor.clone();

        tokio::spawn(async move {
            process(acceptor, stream).await.unwrap();
        });
    }
}

#[instrument(skip_all, name = "get_tls_acceptor", level = "trace")]
fn get_tls_acceptor(
    cert_path: String,
    private_key_path: String,
) -> Result<TlsAcceptor, SpanErr<ClockerError>> {
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

    let config = match rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert, private_key)
    {
        Ok(c) => c,
        Err(e) => return Err(ClockerError::UnexpectedRustls(e).into()),
    };

    Ok(TlsAcceptor::from(Arc::new(config)))
}

#[instrument(skip_all, name = "process", level = "trace")]
async fn process(acceptor: TlsAcceptor, stream: TcpStream) -> Result<(), SpanErr<ClockerError>> {
    let mut tls_stream = match acceptor.accept(stream).await {
        Ok(c) => c,
        Err(e) => return Err(ClockerError::UnexpectedIO(e).into()),
    };

    let mut buf = Vec::with_capacity(4096);
    let _ = match tls_stream.read_buf(&mut buf).await {
        Ok(b) => b,
        Err(e) => return Err(ClockerError::UnexpectedIO(e).into()),
    };

    let msg = match String::from_utf8(buf) {
        Ok(b) => b,
        Err(e) => return Err(ClockerError::UnexpectedFromUtf(e).into()),
    };
    let result = tls_stream.write(msg.as_bytes()).await;

    println!(
        "wrote to stream; msg={:?}, success={:?}",
        msg,
        result.is_ok()
    );

    Ok(())
}
