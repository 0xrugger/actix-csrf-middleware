use actix_csrf_middleware::{
    extract_and_validate_csrf_token, CsrfDoubleSubmitCookie, CsrfMiddleware, CsrfMiddlewareConfig,
    CsrfToken, DEFAULT_CSRF_TOKEN_FIELD,
};
use actix_multipart::Multipart;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use futures_util::StreamExt as _;

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

async fn upload_handler(
    req: HttpRequest,
    csrf: CsrfToken,
    mut payload: Multipart,
) -> actix_web::Result<HttpResponse> {
    extract_and_validate_csrf_token(&req, csrf.0.as_bytes())?;

    // iterate over multipart stream
    while let Some(item) = payload.next().await {
        let mut field = item?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            println!("-- CHUNK: \n{:?}", std::str::from_utf8(&chunk?));
        }
    }

    // Success response
    let html = r#"<!doctype html>
<html>
<head><meta charset="utf-8"><title>Upload success</title></head>
<body>
  <h1>Upload Successful</h1>
</body>
</html>"#;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Example-only secret. Do not use in production.
    let secret = b"example-secret-key-please-change-32+bytes";
    let csrf_config = CsrfMiddlewareConfig::double_submit_cookie(secret)
        .with_multipart(true)
        .with_token_cookie_config(CsrfDoubleSubmitCookie {
            http_only: false,
            secure: false, // false for local dev over plain HTTP
            same_site: actix_web::cookie::SameSite::Strict,
        });

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
