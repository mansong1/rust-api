pub mod configuration;

use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use std::net::TcpListener;
use tera::{Context, Tera};

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World!");
    format!("Hello {}!", &name)
}

async fn render_tmpl(data: web::Data<AppData>, req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap();
    let mut ctx = Context::new();
    ctx.insert("name", name);
    let rendered = data.tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

struct AppData {
    tmpl: Tera,
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        App::new()
            .data(AppData { tmpl: tera })
            .service(web::resource("/tmpl/{name}").route(web::get().to(render_tmpl)))
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
