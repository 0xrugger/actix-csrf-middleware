use actix_csrf_middleware::{
    CsrfMiddleware, CsrfMiddlewareConfig, CsrfToken, DEFAULT_CSRF_TOKEN_FIELD,
};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn render_form(csrf: CsrfToken) -> impl Responder {
    let html = format!(
        r#"<!doctype html>
<html>
<head><meta charset="utf-8"><title>Multipart CSRF example</title></head>
<body>
  <h1>Multipart Upload with CSRF</h1>
  <form method="post" action="/upload" enctype="multipart/form-data">
    <input type="hidden" name="{field}" value="{val}" />
    <div><label>File: <input type="file" name="file" /></label></div>
    <button type="submit">Upload</button>
  </form>
</body>
</html>"#,
        field = DEFAULT_CSRF_TOKEN_FIELD,
        val = csrf.0
    );

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
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