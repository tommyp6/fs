use crate::handlers::core;

use actix_web::web;

pub fn register_handlers(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/").service(
            web::resource("")
                .route(web::get().to(core::index))
                .route(web::post().to(core::upload_file))
                .name("index"),
        ),
    );
}
