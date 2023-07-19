use hyper::{http, Method};

use tower_http::cors::{Any, CorsLayer};

pub fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_headers([http::header::CONTENT_TYPE])
        .allow_origin(Any)
}