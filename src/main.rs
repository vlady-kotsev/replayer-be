use replayer_be::{App, AppResult};

#[tokio::main]
async fn main() -> AppResult<()> {
    App::run().await
}
