
use futures::stream::StreamExt;

use async_std::net::TcpListener;
use async_std::fs::File;
use async_native_tls::TlsAcceptor;

use async_https_template::*;

use local_ip_address::local_ip;


#[async_std::main]
async fn main() {


    let my_local_ip = local_ip().unwrap().to_string();

    let listener = TcpListener::bind(format!("{}:443", &my_local_ip)).await.unwrap();

    let key = File::open("identity.pfx").await.unwrap();
    let acceptor = TlsAcceptor::new(key, "sylane").await.unwrap();

    println!(
        "Server Lauched on IP : {} , waiting for connections...",
        &my_local_ip
    );

    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let acceptor = acceptor.clone();
        let stream = stream.unwrap();
        async_std::task::spawn(async move {
            let stream = acceptor.accept(stream).await.unwrap();
            handle_client(stream).await;
        });
    }
}
