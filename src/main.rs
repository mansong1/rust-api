use rust_api::configuration::get_config;
use rust_api::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to read configuration.");
    /*throw io::error if we fail to bind.
    Otherwise call .await on our Server
    */
    let address = format!("127.0.0.1:{}", config.application.port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
