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
use tracing_spanned::SpanErr;

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
async fn main() -> Result<(), SpanErr<ClockerError>> {
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

fn get_tls_acceptor(
    cert_path: String,
    private_key_path: String,
) -> Result<TlsAcceptor, SpanErr<ClockerError>> {
    let Ok(mut cert_file) = File::open(cert_path) else {
        return Err(ClockerError::Unexpected.into());
    };

    let Ok(cert) =
        rustls_pemfile::certs(&mut BufReader::new(&mut cert_file)).collect::<Result<Vec<_>, _>>()
    else {
        return Err(ClockerError::Unexpected.into());
    };

    let Ok(mut private_key_file) = File::open(private_key_path) else {
        return Err(ClockerError::Unexpected.into());
    };

    let Ok(Some(private_key)) =
        rustls_pemfile::private_key(&mut BufReader::new(&mut private_key_file))
    else {
        return Err(ClockerError::Unexpected.into());
    };

    let Ok(config) = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert, private_key)
    else {
        return Err(ClockerError::Unexpected.into());
    };

    Ok(TlsAcceptor::from(Arc::new(config)))
}

async fn process(acceptor: TlsAcceptor, stream: TcpStream) -> Result<(), SpanErr<ClockerError>> {
    let Ok(mut tls_stream) = acceptor.accept(stream).await else {
        return Err(ClockerError::Unexpected.into());
    };

    let mut buf = Vec::with_capacity(4096);
    let Ok(_) = tls_stream.read_buf(&mut buf).await else {
        return Err(ClockerError::Unexpected.into());
    };

    let Ok(msg) = String::from_utf8(buf) else {
        return Err(ClockerError::Unexpected.into());
    };
    let result = tls_stream.write(msg.as_bytes()).await;

    println!(
        "wrote to stream; msg={:?}, success={:?}",
        msg,
        result.is_ok()
    );

    Ok(())
}
