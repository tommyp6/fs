#![feature(type_alias_impl_trait)]

use actix_files::Files;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    middleware::{Logger, NormalizePath},
    web, App, HttpServer,
};
use dotenv::dotenv;
use lazy_static::lazy_static;
use tera::Tera;

mod config;
mod errors;
mod handlers;
mod router;
mod session;

use crate::config::Config;

lazy_static! {
    pub static ref CONFIG: Config = Config::from_env();
}

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
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    log::info!(
        "Starting server at http://{}:{}",
        CONFIG.host.clone(),
        CONFIG.port
    );

    let key = actix_web::cookie::Key::from(CONFIG.session_key.as_bytes());

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
            .service(Files::new("/uploads", CONFIG.uploads_dir.clone()))
    })
    .bind((CONFIG.host.clone(), CONFIG.port))?
    .run()
    .await
}
