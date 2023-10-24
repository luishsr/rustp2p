use serde::{Deserialize, Serialize};
use tokio::{net::{TcpListener, TcpStream, UdpSocket}, sync::RwLock};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const BROADCAST_ADDR: &str = "255.255.255.255:8888";
const TCP_PORT: u16 = 9000;

#[derive(Debug, Serialize, Deserialize)]
enum Message {
    Handshake { node_name: String, tcp_addr: SocketAddr },
    Greeting,
    Heartbeat,
    HeartbeatResponse,
}

struct NodeInfo {
    last_seen: std::time::Instant,
    tcp_addr: SocketAddr,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let local_addr: SocketAddr = "0.0.0.0:0".parse()?;
    let socket = UdpSocket::bind(&local_addr).await?;
    socket.set_broadcast(true)?;

    let nodes = Arc::new(RwLock::new(HashMap::<String, NodeInfo>::new()));

    // Use Arc to share the socket among tasks.
    let socket = Arc::new(socket);
    let socket_for_broadcast = socket.clone();

    tokio::spawn(async move {
        let tcp_addr = format!("{}:{}", "0.0.0.0", TCP_PORT).parse().unwrap();
        let msg = Message::Handshake {
            node_name: "Node1".to_string(),
            tcp_addr,
        };
        let serialized_msg = serde_json::to_string(&msg).unwrap();

        loop {
            println!("Sending UDP broadcast...");
            socket_for_broadcast.send_to(serialized_msg.as_bytes(), BROADCAST_ADDR).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    });

    let nodes_clone = nodes.clone();
    tokio::spawn(async move {
        let listener = TcpListener::bind(("0.0.0.0", TCP_PORT)).await.unwrap();
        println!("TCP listener started.");
        while let Ok((stream, _)) = listener.accept().await {
            println!("Accepted new TCP connection.");
            tokio::spawn(handle_tcp_stream(stream, nodes_clone.clone()));
        }
    });

    let mut buf = vec![0u8; 1024];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("Received data on UDP from {}", addr);
        let received_msg: Message = serde_json::from_slice(&buf[..len])?;

        if let Message::Handshake { node_name, tcp_addr } = received_msg {
            println!("Received handshake from: {}", node_name);
            {
                let mut nodes_guard = nodes.write().await;
                nodes_guard.insert(node_name.clone(), NodeInfo { last_seen: std::time::Instant::now(), tcp_addr });
            }

            let greeting = Message::Greeting;
            let serialized_greeting = serde_json::to_string(&greeting).unwrap();
            socket.send_to(serialized_greeting.as_bytes(), &addr).await?;

            // Start heartbeat for this node
            let nodes_clone = nodes.clone();
            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    println!("Sending heartbeat to {}", tcp_addr);
                    let mut stream = TcpStream::connect(tcp_addr).await.unwrap();
                    let heartbeat_msg = Message::Heartbeat;
                    let serialized_msg = serde_json::to_string(&heartbeat_msg).unwrap();
                    stream.write_all(serialized_msg.as_bytes()).await.unwrap();
                }
            });
        }
    }
}

async fn handle_tcp_stream(mut stream: TcpStream, nodes: Arc<RwLock<HashMap<String, NodeInfo>>>) {
    let mut buf = vec![0u8; 1024];
    let len = stream.read(&mut buf).await.unwrap();
    let received_msg: Message = serde_json::from_slice(&buf[..len]).unwrap();

    match received_msg {
        Message::Heartbeat => {
            println!("Received Heartbeat");
            let response = Message::HeartbeatResponse;
            let serialized_response = serde_json::to_string(&response).unwrap();
            stream.write_all(serialized_response.as_bytes()).await.unwrap();
        },
        _ => {}
    }
}
