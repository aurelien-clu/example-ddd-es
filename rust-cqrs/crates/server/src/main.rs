#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod metadata_extension;
mod routes;

use axum::extract::Extension;
use axum::routing::get;
use axum::Router;

use domain_bank::config::cqrs_framework;
use postgres_es::default_postgress_pool;

#[tokio::main]
async fn main() {
    // Configure the CQRS framework, backed by a Postgres database, along with two queries:
    // - a simply-query prints events to stdout as they are published
    // - `account_query` stores the current state of the account in a ViewRepository that we can access
    //
    // The needed database tables are automatically configured with `docker-compose up -d`,
    // see init file at `/db/init.sql` for more.
    let pool = default_postgress_pool("postgresql://demo_user:demo_pass@localhost:5432/demo").await;
    let (cqrs, account_query) = cqrs_framework(pool);

    // Configure the Axum routes and services.
    // For this example a single logical endpoint is used and the HTTP method
    // distinguishes whether the call is a command or a query.
    let router = Router::new()
        .route(
            "/account/:account_id",
            get(routes::bank::query_handler).post(routes::bank::command_handler),
        )
        .layer(Extension(cqrs))
        .layer(Extension(account_query));

    // Start the Axum server.
    axum::Server::bind(&"0.0.0.0:3030".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
