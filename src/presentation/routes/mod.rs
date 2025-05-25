use axum::Router;
use tower_http::cors::CorsLayer;

pub fn create_router() -> Router {
    Router::new().layer(CorsLayer::permissive())
}
