use clap::Parser;
use dotenvy::dotenv;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(short = 'p', long = "port", env, default_value = "3000")]
    port: u16,
}

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let args = Args::parse();

    dbg!(args.port);

    let addr = format!("0.0.0.0:{}", args.port);
    let listener = TcpListener::bind(addr).await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            process(stream).await;
        });
    }
}

async fn process(mut stream: TcpStream) {
    let mut buf = Vec::with_capacity(4096);
    stream.read_buf(&mut buf).await.unwrap();

    let msg = String::from_utf8(buf).expect("failed to convert String");
    let result = stream.write(msg.as_bytes()).await;

    println!(
        "wrote to stream; msg={:?}, success={:?}",
        msg,
        result.is_ok()
    );
}
