use std::error::Error;


#[tokio::main]
async fn main() {
    if let Err(e) = roxy::run().await {
        log::error!("something went wrong: {}", e)
    }
}
