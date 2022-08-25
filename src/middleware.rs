use crate::{
    session::{flash, FlashMessage},
    CONFIG,
};
use actix_session::SessionExt;
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::{
        header::{HeaderValue, LOCATION},
        StatusCode,
    },
    Error, HttpResponse,
};
use futures_util::TryStreamExt;
use human_bytes::human_bytes;
use std::{
    future::{ready, Future, Ready},
    pin::Pin,
    rc::Rc,
};

#[derive(Debug)]
pub struct ContentLengthChecker;

impl ContentLengthChecker {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S, B> Transform<S, ServiceRequest> for ContentLengthChecker
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = ContentLengthCheckerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ContentLengthCheckerMiddleware {
            service: Rc::new(service),
        }))
    }
}

#[derive(Debug)]
pub struct ContentLengthCheckerMiddleware<S> {
    service: Rc<S>,
}

fn get_content_length(header: Option<&HeaderValue>) -> u64 {
    if let Some(header) = header {
        header.to_str().unwrap_or("0").parse().unwrap_or(0)
    } else {
        0
    }
}

impl<S, B> Service<ServiceRequest> for ContentLengthCheckerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        #[allow(non_snake_case)]
        let MAX_ALLOWED_CONTENT_LENGTH: u64 = CONFIG.max_file_size;
        let service = Rc::clone(&self.service);

        let content_length = match req.headers().get("content-length") {
            None => 0,
            Some(content_length) => get_content_length(Some(content_length)),
        };

        if content_length <= MAX_ALLOWED_CONTENT_LENGTH {
            return Box::pin(async move {
                service
                    .call(req)
                    .await
                    .map(ServiceResponse::map_into_left_body)
            });
        }

        Box::pin(async move {
            // Drain body because of #2695. (https://github.com/actix/actix-web/issues/2695)
            let (r, mut payload) = req.into_parts();
            while let Ok(Some(_)) = payload.try_next().await {}
            let session = r.get_session();

            flash(
                &session,
                FlashMessage::error(format!(
                    "Your file exceeds the maximum file size of {}",
                    human_bytes(MAX_ALLOWED_CONTENT_LENGTH as f64)
                )),
            )?;

            let req = ServiceRequest::from_parts(r, payload);

            let mut res = HttpResponse::new(StatusCode::FOUND);

            res.headers_mut()
                .insert(LOCATION, HeaderValue::from_static("/"));

            Ok(req.into_response(res.map_into_right_body()))
        })
    }
}
