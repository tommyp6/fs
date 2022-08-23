#[macro_export]
macro_rules! render_template {
    ($tera:expr, $session:expr, $ctx:expr, $name:literal) => {
        $ctx.insert("_flashes", &get_flash_messages($session)?);

        Ok(HttpResponse::Ok().content_type("text/html").body(
            $tera
                .render($name, $ctx)
                .map_err(|_| error::ErrorInternalServerError("Template error"))?,
        ))
    };

    ($tera:expr, $session:expr, $name:literal) => {{
        let mut ctx = Context::new();

        ctx.insert("_flashes", &get_flash_messages($session)?);

        Ok(HttpResponse::Ok().content_type("text/html").body(
            $tera
                .render($name, &ctx)
                .map_err(|_| error::ErrorInternalServerError("Template error"))?,
        ))
    }};
}

#[macro_export]
macro_rules! redirect {
    ($location:literal) => {
        Ok(HttpResponse::Ok()
            .status(actix_web::http::StatusCode::FOUND)
            .append_header(("Location", $location))
            .finish())
    };

    ($req:expr, $location:literal) => {
        Ok(HttpResponse::Ok()
            .status(actix_web::http::StatusCode::FOUND)
            .append_header(("Location", $req.url_for_static($location)?.as_str()))
            .finish())
    };

    ($req:expr, $location:literal, $params:expr) => {
        Ok(HttpResponse::Ok()
            .status(actix_web::http::StatusCode::FOUND)
            .append_header(("Location", $req.url_for($location, $params)?.as_str()))
            .finish())
    };
}
