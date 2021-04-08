use rust_api::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /*throw io::error if we fail to bind.
    Otherwise call .await on our Server 
    */
    run()?.await
}