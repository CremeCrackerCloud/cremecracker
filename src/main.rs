#[actix_web::main]
async fn main() -> std::io::Result<()> {
    paas_api::run().await
}
