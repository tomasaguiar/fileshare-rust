mod client;
mod server;

use crate::client::client::client_connection;
use crate::server::server::server_connection;

fn main() {
    // Start the server
    server_connection();

    // Start the client
    client_connection();

    println!("Engine running...");
}
