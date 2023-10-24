# rustp2p

A simple yet powerful Peer-to-Peer key-value database implemented in Rust. This project also includes a CLI (Command Line Interface) that enables users to interact with the database effortlessly. Explore the power of distributed systems, emphasising the performance and reliability that Rust brings to the table.

Features

  UDP Handshake: Discover peers efficiently using UDP broadcasts.
  TCP Communication: Ensures reliable communication between nodes.
  Concurrent Access: Multi-threaded approach leveraging Tokio.
  Simple Key-Value Store: A straightforward data model that can be extended for more complex use cases.
  Command Line Interface (CLI): Directly interact with the database, set or get values, and manage nodes.

Getting Started

Ensure you have Rust and Cargo installed. If not, get them from here.
Installation

  Clone the repository:

    git clone https://github.com/username/rust-p2p-database.git

  Navigate into the directory and build the project:

    cd rust-p2p-database
    cargo build --release

Usage

  Run the binary:

    ./target/release/rust-p2p-database

The node will automatically discover and communicate with other nodes on the same network.

CLI Commands

After starting the node, you can use the following commands to interact with the database:

  Set Value: Store a key-value pair in the database.

    set <key> <value>

Get Value: Retrieve the value associated with a given key.

    get <key>

Remember to replace <key> and <value> with your desired key and value.

Example:
  
    ./rustdbcli set --node 0.0.0.0:9000 "testKey" "testValue"

Contribution

Pull requests are welcome! For significant changes, please open an issue first to discuss the proposed changes. Also, ensure that your contributions adhere to Rust's standard coding conventions.
