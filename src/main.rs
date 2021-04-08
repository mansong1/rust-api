use rust_api::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /*throw io::error if we fail to bind.
    Otherwise call .await on our Server
    */
    let address = TcpListener::bind("127.0.0.1:8000")?;
    run(address)?.await
}
