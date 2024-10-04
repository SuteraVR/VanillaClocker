mod cert;
mod err;

use cert::{read_cert_file, read_private_key_file};
use clap::Parser;
use dotenvy::dotenv;
use err::ClockerError;
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

#[instrument(skip_all, name = "initialize_tracing_subscriber", level = "trace")]
fn initialize_tracing_subscriber() -> Result<(), SpanErr<ClockerError>> {
    tracing_subscriber::Registry::default()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_filter(tracing_subscriber::filter::LevelFilter::INFO),
        )
        .with(ErrorLayer::default())
        .try_init()
        .map_err(ClockerError::InitializeTracingSubscriber)?;

    Ok(())
}

#[tokio::main]
#[instrument(skip_all, name = "main", level = "trace")]
async fn main() -> Result<(), SpanErr<ClockerError>> {
    initialize_tracing_subscriber()?;

    let _ = dotenv();

    let args = Args::parse();

    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, args.port))
        .await
        .map_err(|e| ClockerError::CreateTCPListener(e, args.port))?;

    let acceptor = get_tls_acceptor(args.cert_path, args.private_key_path)?;

    loop {
        let (stream, _) = listener
            .accept()
            .await
            .map_err(ClockerError::AcceptNewConnection)?;

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
    let cert = read_cert_file(cert_path)?;

    let private_key = read_private_key_file(private_key_path)?;

    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert, private_key)
        .map_err(ClockerError::UnexpectedRustls)?;

    Ok(TlsAcceptor::from(Arc::new(config)))
}

#[instrument(skip_all, name = "process", level = "trace")]
async fn process(acceptor: TlsAcceptor, stream: TcpStream) -> Result<(), SpanErr<ClockerError>> {
    let mut tls_stream = acceptor
        .accept(stream)
        .await
        .map_err(ClockerError::UnexpectedIO)?;

    let mut buf = Vec::with_capacity(4096);
    let _ = tls_stream
        .read_buf(&mut buf)
        .await
        .map_err(ClockerError::UnexpectedIO)?;

    let msg = String::from_utf8(buf).map_err(ClockerError::UnexpectedFromUtf)?;
    let result = tls_stream.write(msg.as_bytes()).await;

    println!(
        "wrote to stream; msg={:?}, success={:?}",
        msg,
        result.is_ok()
    );

    Ok(())
}
