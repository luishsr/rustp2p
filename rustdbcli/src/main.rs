use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Debug, Serialize, Deserialize)]
enum Message {
    // ... existing variants
    SetValue { key: String, value: String },
    GetValue { key: String },
    ValueResponse { value: Option<String> },
}


#[derive(StructOpt, Debug)]
#[structopt(name = "node-cli", about = "CLI for interacting with the P2P network.")]
enum Cli {
    /// Set a key-value pair on a specific node
    Set {
        #[structopt(short, long)]
        node: String,
        key: String,
        value: String,
    },
    /// Get value by key from a specific node
    Get {
        #[structopt(short, long)]
        node: String,
        key: String,
    },
}

async fn handle_cli_command(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli {
        Cli::Set { node, key, value } => {
            let addr: SocketAddr = node.parse()?;
            let mut stream = TcpStream::connect(addr).await?;
            let msg = Message::SetValue { key, value };
            let serialized_msg = serde_json::to_string(&msg).unwrap();
            stream.write_all(serialized_msg.as_bytes()).await?;

            let mut buf = vec![0u8; 1024];
            let len = stream.read(&mut buf).await?;
            let response: Message = serde_json::from_slice(&buf[..len])?;
            match response {
                Message::ValueResponse { value: Some(resp) } => println!("{}", resp),
                _ => println!("Unexpected response from node."),
            }
        }
        Cli::Get { node, key } => {
            let addr: SocketAddr = node.parse()?;
            let mut stream = TcpStream::connect(addr).await?;
            let msg = Message::GetValue { key };
            let serialized_msg = serde_json::to_string(&msg).unwrap();
            stream.write_all(serialized_msg.as_bytes()).await?;

            let mut buf = vec![0u8; 1024];
            let len = stream.read(&mut buf).await?;
            let response: Message = serde_json::from_slice(&buf[..len])?;
            match response {
                Message::ValueResponse { value: Some(resp) } => println!("{}", resp),
                Message::ValueResponse { value: None } => println!("Key not found."),
                _ => println!("Unexpected response from node."),
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Cli::from_args();
    handle_cli_command(opt).await
}
