use actix_cors::Cors;
use actix_web::http;
use actix_web::http::Cookie;
use actix_web::web;
use actix_web::App;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;

const AUTO_LOGIN_COOKIE_NAME: &str = "pmo-auto-login";

enum AutoLoginOptions {
    Never,
    Always,
}

impl From<AutoLoginOptions> for &str {
    fn from(o: AutoLoginOptions) -> Self {
        match o {
            AutoLoginOptions::Never => "0",
            AutoLoginOptions::Always => "1",
        }
    }
}

fn set_cookie(name: &'static str, value: &'static str) -> HttpResponse {
    HttpResponse::Ok()
        .cookie(
            Cookie::build(name, value)
                .path("/")
                .secure(true)
                .http_only(true)
                .finish(),
        )
        .finish()
}

async fn enable_auto_login(_: HttpRequest) -> impl Responder {
    set_cookie(AUTO_LOGIN_COOKIE_NAME, AutoLoginOptions::Always.into())
}

async fn disable_auto_login(_: HttpRequest) -> impl Responder {
    set_cookie(AUTO_LOGIN_COOKIE_NAME, AutoLoginOptions::Never.into())
}

async fn healthz() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["GET", "DELETE", "HEAD"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .service(web::resource("/healthz").route(web::head().to(healthz)))
            .service(
                web::resource("/_c/auto-login")
                    .route(web::get().to(enable_auto_login))
                    .route(web::delete().to(disable_auto_login)),
            )
    })
    .bind("0.0.0.0:8086")?
    .run()
    .await
}
