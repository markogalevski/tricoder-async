use std::{
    net::{SocketAddr, ToSocketAddrs},
    time::Duration,
};

use crate::{common_ports::MOST_COMMON_PORTS_100, modules::Port, modules::Subdomain};
use futures::StreamExt;
use tokio::net::TcpStream;

pub async fn scan_ports(port_concurrency: usize, mut subdomain: Subdomain) -> Subdomain {
    let socket_addresses: Vec<SocketAddr> = format!("{}:1024", subdomain.domain)
        .to_socket_addrs()
        .expect("Port scanner: creating socket addresses")
        .collect();
    if socket_addresses.is_empty() {
        return subdomain;
    }
    let socket_address = socket_addresses[0];
    let (to_check_port_sender, to_check_port_receiver) =
        tokio::sync::mpsc::channel(port_concurrency);
    let (open_port_sender, open_port_receiver) = tokio::sync::mpsc::channel(port_concurrency);

    tokio::spawn(async move {
        for i in MOST_COMMON_PORTS_100 {
            let _ = to_check_port_sender.send(*i).await;
        }
    });

    let to_check_port_stream = tokio_stream::wrappers::ReceiverStream::new(to_check_port_receiver);
    to_check_port_stream
        .for_each_concurrent(port_concurrency, |port| {
            let open_port_sender = open_port_sender.clone();
            async move {
                let port = scan_port(socket_address, port).await;
                if port.is_open {
                    let _ = open_port_sender.send(port).await;
                }
            }
        })
        .await;
    drop(open_port_sender);

    let open_port_stream = tokio_stream::wrappers::ReceiverStream::new(open_port_receiver);
    subdomain.open_ports = open_port_stream.collect().await;
    subdomain
}

async fn scan_port(mut socket_addr: SocketAddr, port: u16) -> Port {
    let timeout = Duration::from_secs(3);
    socket_addr.set_port(port);

    let is_open = matches!(
        tokio::time::timeout(timeout, TcpStream::connect(&socket_addr)).await,
        Ok(Ok(_)),
    );
    Port { port, is_open }
}
