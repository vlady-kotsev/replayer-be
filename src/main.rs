use anyhow::Result;
use replayer_be::App;

#[tokio::main]
async fn main() -> Result<()> {
    App::run().await?;

    Ok(())
}
