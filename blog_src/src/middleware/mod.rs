pub mod auth;

use axum::extract::State;
use axum::middleware::{self, Next};
use axum::response::Response;
use axum::http::Request;

use crate::AppState;

pub async fn pod_name_header(
    State(state): State<AppState>,
    request: Request<axum::body::Body>, 
    next: Next,
) -> Response {
    let mut response = next.run(request).await;
    let pod_name = state.pod_name.clone();
    response.headers_mut().insert(
        "X-Pod-Name",
        pod_name.parse().unwrap()
    );
    response
}