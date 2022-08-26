use crate::handlers::core;

use actix_web::web;

pub fn register_handlers(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(core::index))
        .route("/upload", web::post().to(core::upload_file));
}
