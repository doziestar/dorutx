use axum::{
    routing::get,
    Router,
};
use crate::routes;
pub(crate) async fn server() {
    let app = Router::new()
        .route("/", get(routes::home));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
