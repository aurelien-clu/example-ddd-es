use std::sync::Arc;

use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use cqrs_es::persist::ViewRepository;
use postgres_es::{PostgresCqrs, PostgresViewRepository};

use crate::metadata_extension::MetadataExtension;
use domain_bank::domain::aggregate::BankAccount;
use domain_bank::domain::commands::BankAccountCommand;
use domain_bank::queries::BankAccountView;

// Serves as our query endpoint to respond with the materialized `BankAccountView`
// for the requested account.
pub async fn query_handler(
    Path(account_id): Path<String>,
    Extension(view_repo): Extension<Arc<PostgresViewRepository<BankAccountView, BankAccount>>>,
) -> Response {
    let view = match view_repo.load(&account_id).await {
        Ok(view) => view,
        Err(err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
        }
    };
    match view {
        None => StatusCode::NOT_FOUND.into_response(),
        Some(account_view) => (StatusCode::OK, Json(account_view)).into_response(),
    }
}

// Serves as our command endpoint to make changes in a `BankAccount` aggregate.
pub async fn command_handler(
    Path(account_id): Path<String>,
    Extension(cqrs): Extension<Arc<PostgresCqrs<BankAccount>>>,
    MetadataExtension(metadata): MetadataExtension,
    Json(command): Json<BankAccountCommand>,
) -> Response {
    match cqrs
        .execute_with_metadata(&account_id, command, metadata)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}
