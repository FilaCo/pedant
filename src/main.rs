use pedant::v1::*;

#[tokio::main]
async fn main() -> PedantResult<()> {
    Pedant::default().run().await
}
