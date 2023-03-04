use std::error::Error;

use roxy::proxy::Arguments;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    roxy::logger::init();
    roxy::proxy::start(Arguments { port: 3000 }).await?;
    Ok(())
}
