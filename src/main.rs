use imp::v1::*;

#[tokio::main]
async fn main() -> AppResult<()> {
    Imp::default().run().await
}
