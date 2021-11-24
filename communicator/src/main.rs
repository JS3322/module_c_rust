extern crate communicator;
mod client;
mod network::server;

fn main() {
    communicator::client::conntect();
    communicator::network::server::conntect();
    println!("Hello, world!");
}
