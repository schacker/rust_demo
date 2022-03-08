use actix_files as fs;
use actix_session::{CookieSession, Session};
use actix_utils::mpsc;
use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    error, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Result
};
use bytes::Bytes;

/// favicon handler
#[get("/favicon")]
pub async fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("../static/favicon.ico")?)
}

/// simple index handler9
#[get("/welcome")]
pub async fn welcome(session: Session, req: HttpRequest) -> HttpResponse {
    println!("{:?}", req);

    // session
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter").unwrap() {
        println!("SESSION value: {}", count);
        counter = count + 1;
    }

    // set counter to session
    session.set("counter", counter).unwrap();

    // response
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/welcome.html"))
}

/// 404 handler
pub async fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("../static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

/// response body
pub async fn response_body(path: web::Path<String>) -> HttpResponse {
    let text = format!("Hello {}!", *path);

    let (tx, rx_body) = mpsc::channel();
    let _ = tx.send(Ok::<_, Error>(Bytes::from(text)));

    HttpResponse::Ok().streaming(rx_body)
}

/// handler with path parameters like `/user/{name}/`
pub async fn with_param(req: HttpRequest, path: web::Path<(String,)>) -> HttpResponse {
  println!("{:?}", req);

  HttpResponse::Ok()
      .content_type("text/plain")
      .body(format!("Hello {:?}!", path.0))
}