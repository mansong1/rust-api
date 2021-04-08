use rust_api::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /*throw io::error if we fail to bind.
    Otherwise call .await on our Server 
    */
    run("127.0.0.1:8000")?.await
}