#![forbid(unsafe_code)]
#![deny(clippy::all)]
mod metadata_extension;
mod routes;

use axum::extract::Extension;
use axum::routing::get;
use axum::Router;

use postgres_es::default_postgress_pool;

#[tokio::main]
async fn main() {
    let pool = default_postgress_pool("postgresql://demo_user:demo_pass@localhost:5432/demo").await;
    let (cqrs, current_journey_query) = domain_plane::config::cqrs_framework(pool);

    let router = Router::new()
        .route(
            "/plane/:registration_id",
            get(routes::plane::query_handler).post(routes::plane::command_handler),
        )
        .layer(Extension(cqrs))
        .layer(Extension(current_journey_query));

    axum::Server::bind(&"0.0.0.0:3030".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
