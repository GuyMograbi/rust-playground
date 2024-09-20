use axum::{body::Body, http::Request, middleware::Next, response::Response};
use log;

pub async fn logging_middleware(req: Request<Body>, next: Next) -> Response {
    println!("middleware ! Request: {:#?}", req);
    let method = req.method().clone();
    let uri = req.uri().clone();

    // Log the request method and path
    log::info!("middleware ! Received request: {} {}", method, uri);

    // Call the next handler
    let response = next.run(req).await;

    // Log the response status
    log::info!("Responded: {} {} - {}", method, uri, response.status());

    response
}
