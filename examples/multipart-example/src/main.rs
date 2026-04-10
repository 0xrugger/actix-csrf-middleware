use actix_csrf_middleware::{CsrfMiddleware, CsrfMiddlewareConfig};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn render_form() -> impl Responder {
    HttpResponse::Ok().body("TODO")
}

async fn upload_handler() -> impl Responder {
    HttpResponse::Ok().body("TODO")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Example-only secret. Do not use in production.
    let secret = b"example-secret-key-please-change-32+bytes";
    let csrf_config = CsrfMiddlewareConfig::double_submit_cookie(secret)
        .with_multipart(true);

    println!("Starting actix web at http://localhost:8080...");

    HttpServer::new(move || {
        App::new()
            .wrap(CsrfMiddleware::new(csrf_config.clone()))
            .route("/", web::get().to(render_form))
            .route("/upload", web::post().to(upload_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}