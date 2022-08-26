use crate::{session::get_flash_messages, CONFIG};
use actix_multipart::Multipart;
use actix_session::Session;
use actix_web::{error, web, Error, HttpRequest, HttpResponse, Responder, Result};
use fs::render_template;
use futures_util::TryStreamExt;
use tera::{Context, Tera};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

type Response = Result<impl Responder, Error>;

pub async fn index(tera: web::Data<Tera>, session: Session) -> Response {
    render_template!(tera, &session, "index.html")
}

pub async fn upload_file(_req: HttpRequest, mut payload: Multipart, _session: Session) -> Response {
    // Deal with dup. files
    // let mut has_flashed = false;

    let mut filepath: Option<String> = None;

    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();

        let filename = content_disposition
            .get_filename()
            .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);

        let path = format!("{}/{filename}", CONFIG.uploads_dir);

        if !filepath.is_some() {
            filepath = Some(path.clone());
        }

        // if !has_flashed {
        //     flash(
        //         &session,
        //         FlashMessage::ok_safe(format!(
        //             "Uploaded file to <a href=/{0}>{0}</a>",
        //             filepath.clone()
        //         )),
        //     )?;
        //     has_flashed = true;
        // }

        let mut f = tokio::fs::File::create(path).await?;

        while let Some(chunk) = field.try_next().await? {
            // TODO: Report errors while writing to file.
            f.write_all(&chunk).await?;
        }
    }

    let body = format!("Uploaded file to <a href=/{0}>{0}</a>", filepath.unwrap());
    Ok(HttpResponse::Ok().content_type("text/plain").body(body))

    //    redirect!(req, "index")
}
