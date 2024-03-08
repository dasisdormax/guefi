use std::env;

use guefi_worker::worker;

#[tokio::main]
async fn main() {
    // TODO: Receive key through stdin
    let key = env::args().nth(1).expect("First argument is server key");

    worker::run_client(key).await;
}