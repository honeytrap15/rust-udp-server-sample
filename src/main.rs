extern crate env_logger as logger;
extern crate log;

use async_std::net::UdpSocket;
use futures::future::join_all;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    logger::init();

    let mut futures = Vec::new();

    for n in 0..100 {
        let port = 10000 + n;
        let sock = match UdpSocket::bind(format!("0.0.0.0:{}", port)).await {
            Ok(sock) => sock,
            Err(e) => {
                log::error!("failed to bind socket: {}, {}", port, e);
                continue;
            }
        };
        log::info!("start udp server: {}", port);

        futures.push(tokio::spawn(async move {
            let mut buf = vec![0u8; 1024];
            loop {
                match sock.recv_from(&mut buf).await {
                    Ok((size, addr)) => {
                        log::info!("recv from: {}, {:?}", addr, &buf[0..size]);
                    }
                    Err(e) => {
                        log::error!("failed to recv data: {}", e);
                        continue;
                    }
                };
            }
        }));
    }

    join_all(futures).await;
    Ok(())
}

#[test]
fn test_client() {
    for n in 0..100 {
        let sock = std::net::UdpSocket::bind("127.0.0.1:0").unwrap();
        sock.set_write_timeout(Some(std::time::Duration::from_secs(2)))
            .unwrap();
        sock.set_read_timeout(Some(std::time::Duration::from_secs(2)))
            .unwrap();
        sock.send_to(b"hello", format!("0.0.0.0:{}", 10000 + n))
            .unwrap();
    }
}
