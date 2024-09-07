use clap::Parser;
use dotenvy::dotenv;
use rustls_pemfile;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::{rustls, TlsAcceptor};

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
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().expect(".env file not found");
    let args = Args::parse();

    let addr = format!("0.0.0.0:{}", args.port);
    let listener = TcpListener::bind(addr).await?;

    let acceptor =
        get_tls_acceptor(args.cert_path, args.private_key_path).expect("get tls acceptor error");

    loop {
        let (stream, _) = listener.accept().await?;
        let acceptor = acceptor.clone();

        tokio::spawn(async move {
            process(acceptor, stream).await.unwrap();
        });
    }
}

fn get_tls_acceptor(
    cert_path: String,
    private_key_path: String,
) -> Result<TlsAcceptor, Box<dyn Error>> {
    let cert_file = rustls_pemfile::certs(&mut BufReader::new(&mut File::open(cert_path)?))
        .collect::<Result<Vec<_>, _>>()?;

    let private_key_file =
        rustls_pemfile::private_key(&mut BufReader::new(&mut File::open(private_key_path)?))?
            .unwrap();

    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_file, private_key_file)?;

    Ok(TlsAcceptor::from(Arc::new(config)))
}

async fn process(acceptor: TlsAcceptor, stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut tls_stream = acceptor.accept(stream).await?;

    let mut buf = Vec::with_capacity(4096);
    tls_stream.read_buf(&mut buf).await?;

    let msg = String::from_utf8(buf)?;
    let result = tls_stream.write(msg.as_bytes()).await;

    println!(
        "wrote to stream; msg={:?}, success={:?}",
        msg,
        result.is_ok()
    );

    Ok(())
}
