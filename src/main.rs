use imp::v1::*;

#[tokio::main]
async fn main() -> AppResult<()> {
    Editor::new().run().await
}
