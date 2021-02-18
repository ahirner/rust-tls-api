#[cfg(feature = "runtime-async-std")]
use async_std::net::TcpStream;
use std::fs;
use tls_api::TlsConnector;
use tls_api::TlsConnectorBuilder;
use tls_api::X509Cert;
use tls_api_test::block_on;
#[cfg(feature = "runtime-tokio")]
use tokio::net::TcpStream;

async fn run() {
    let socket = TcpStream::connect(("127.0.0.1", 4433)).await.unwrap();

    let mut builder = tls_api_openssl::TlsConnector::builder().unwrap();
    builder
        .add_root_certificate(&X509Cert::from_der(fs::read("ca.der").unwrap()).unwrap())
        .unwrap();
    let connector = builder.build().unwrap();
    connector.connect("localhost", socket).await.unwrap();
}

fn main() {
    block_on(run());
}
