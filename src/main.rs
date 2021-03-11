use actix_multipart::Multipart;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use async_std::prelude::*;
use futures::{StreamExt, TryStreamExt};
use uuid::Uuid;

async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut fileurl = String::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filename = content_type
            .get_filename()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;

        let filename = sanitize_filename::sanitize(&filename);
        let file_id = Uuid::new_v4().to_string();

        if fileurl.is_empty() {
            fileurl = format!("/done/{}/{}", file_id, filename);
        }

        async_std::fs::create_dir_all(format!("/app/uploads/{}", file_id)).await?;
        let filepath = format!("/app/uploads/{}/{}", file_id, filename);
        let mut f = async_std::fs::File::create(filepath).await?;

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
    }

    Ok(HttpResponse::Found().header("Location", fileurl).finish())
}

async fn done(param: web::Path<(String, String)>) -> HttpResponse {
    let (file_id, filename) = param.into_inner();
    let fileurl = format!("https://i.ghostplanet.live/{}/{}", file_id, filename);

    let html = format!(
        r##"<html><head><title>File Uploaded</title><link rel="stylesheet" href="https://ghostplanet.live/global.css"></head>
<body>
    <h1>File successfully uploaded!</h1>
    <p>You can visit your uploaded file here:</p><br>
    <a href="{}">{}</a><br>
    <h1><a href="/">Go back...</a></h1>
</body>
</html>
"##,
        fileurl, fileurl
    );

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn index() -> HttpResponse {
    let html = r#"<html><head>
        <title>GPL File Sharing</title>
        <style>
        html {
margin: 0;
top: 0;
left: 0;
background-color: #121212;
color: #fff;
}

h1 {
        text-align: center;
}

a {
color: #50c878;
}

a:visited {
color: #50c878;
}
        </style>
        </head>
        <body>
            <h1>GPL File Sharing</h1>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    async_std::fs::create_dir_all("/app/uploads").await?;

    let ip = "0.0.0.0:8080";

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/")
                    .route(web::get().to(index))
                    .route(web::post().to(save_file)),
            )
            .service(web::resource("/done/{file_id}/{filename}").route(web::get().to(done)))
    })
    .bind(ip)?
    .run()
    .await
}
