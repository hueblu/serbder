use anyhow::Result;
use serbder::*;

#[tokio::main]
async fn main() -> Result<()> {
    let _server = Server::new().await?.run().await?;
    Ok(())
}

// fn index(req: Request) -> Response {}

// fn ping(req: Request) -> Response {}
