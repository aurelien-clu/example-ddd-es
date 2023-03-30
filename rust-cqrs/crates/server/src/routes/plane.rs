use std::sync::Arc;

use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use cqrs_es::persist::ViewRepository;
use postgres_es::{PostgresCqrs, PostgresViewRepository};

use crate::metadata_extension::MetadataExtension;
use domain_plane::domain::aggregate::Plane;
use domain_plane::domain::commands::Command;
use domain_plane::queries::current_journey::CurrentJourneyView;

pub async fn query_handler(
    Path(registration_id): Path<String>,
    Extension(view_repo): Extension<Arc<PostgresViewRepository<CurrentJourneyView, Plane>>>,
) -> Response {
    let view = match view_repo.load(&registration_id).await {
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

pub async fn command_handler(
    Path(registration_id): Path<String>,
    Extension(cqrs): Extension<Arc<PostgresCqrs<Plane>>>,
    MetadataExtension(metadata): MetadataExtension,
    Json(command): Json<Command>,
) -> Response {
    match cqrs
        .execute_with_metadata(&registration_id, command, metadata)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}
