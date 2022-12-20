use anyhow::Result;
use serbder::*;

#[tokio::main]
async fn main() -> Result<()> {
    let _server = Server::new().await?.run().await?;
    Ok(())
}

// async fn index(req: Request) -> Response {}

// async fn ping(req: Request) -> Response {}
