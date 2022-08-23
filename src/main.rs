#![feature(type_alias_impl_trait)]

use actix_files::Files;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    middleware::{Logger, NormalizePath},
    web, App, HttpServer,
};
use lazy_static::lazy_static;
use tera::Tera;

mod errors;
mod handlers;
mod router;
mod session;

// TODO: make site configs for session key, upload path, etc.
static SESSION_SIGNING_KEY: &[u8] = &[0; 64];

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html"]);
        tera
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    log::info!("Starting server at http://localhost:8080");

    let key = actix_web::cookie::Key::from(SESSION_SIGNING_KEY);

    HttpServer::new(move || {
        let session_store = SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
            .cookie_secure(false)
            .build();

        App::new()
            .app_data(web::Data::new(TEMPLATES.clone()))
            .wrap(errors::error_handler())
            .wrap(session_store)
            .wrap(NormalizePath::trim())
            .wrap(Logger::default())
            .configure(router::register_handlers)
            .service(Files::new("/static", "static"))
            .service(Files::new("/uploads", "uploads"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
