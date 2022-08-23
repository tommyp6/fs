use crate::session::{flash, get_flash_messages, FlashKind, FlashMessage};
use actix_multipart::Multipart;
use actix_session::Session;
use actix_web::{error, web, Error, HttpRequest, HttpResponse, Responder, Result};
use fs::{redirect, render_template};
use futures_util::TryStreamExt as _;
use std::io::Write;
use tera::{Context, Tera};
use uuid::Uuid;

type Response = Result<impl Responder, Error>;

pub async fn index(tera: web::Data<Tera>, session: Session) -> Response {
    render_template!(tera, &session, "index.html")
}

pub async fn upload_file(req: HttpRequest, mut payload: Multipart, session: Session) -> Response {
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();

        let filename = content_disposition
            .get_filename()
            .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);

        let filepath = format!("uploads/{filename}");

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        while let Some(chunk) = field.try_next().await? {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }

    flash(&session, FlashMessage::new(FlashKind::OK, "Uploaded file!"))?;

    redirect!(req, "index")
}
