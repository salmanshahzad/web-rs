use axum::{
    http::{header, HeaderValue, Request},
    middleware::Next,
    response::Response,
};

pub async fn cors<B>(req: Request<B>, next: Next<B>) -> Response {
    let mut res = next.run(req).await;
    let headers = res.headers_mut();
    headers.append(
        header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("true"),
    );
    headers.append(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("HEAD,GET,POST,PATCH,PUT,DELETE"),
    );
    headers.append(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("http://localhost:1025"),
    );
    res
}
