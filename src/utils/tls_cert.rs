use futures::StreamExt;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio_rustls_acme::{AcmeConfig, caches::DirCache};
use tokio_stream::wrappers::TcpListenerStream;

// Import your existing env provider
use crate::utils::env_provider;

pub async fn start_tls_server() -> Result<(), Box<dyn std::error::Error>> {
    // Use PUBLIC_DOMAIN from your env provider
    let domain = env_provider::env_var!("PUBLIC_DOMAIN");

    // Define the address to bind to (port 443 for HTTPS)
    let addr: SocketAddr = "[::]:443".parse()?; // Alternatively "0.0.0.0:443"

    // Create TCP listener
    let tcp_listener = TcpListener::bind(addr).await?;
    let tcp_incoming = TcpListenerStream::new(tcp_listener);

    // Configure Let's Encrypt with TLS-ALPN
    let mut tls_incoming = AcmeConfig::new([domain.clone()])
        .contact_push("mailto:alex.m.huber@web.de") // 🔁 Replace with your real email
        .cache(DirCache::new("./acme_cache"))
        .directory_lets_encrypt(true) // Set to false for staging
        .incoming(tcp_incoming, Vec::new());

    println!("🌐 TLS server active on https://{}", domain);

    // Accept connections (here you'd integrate Rocket or your logic)
    while let Some(stream) = tls_incoming.next().await {
        match stream {
            Ok(_tls_stream) => {
                println!("🔐 New secure TLS connection accepted");
                // You would pass _tls_stream to Rocket or a handler
            }
            Err(e) => eprintln!("⚠️ TLS stream error: {}", e),
        }
    }

    Ok(())
}
