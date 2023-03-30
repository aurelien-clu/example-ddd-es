use std::sync::Arc;

use cqrs_es::Query;
use postgres_es::{PostgresCqrs, PostgresViewRepository};
use sqlx::{Pool, Postgres};

use crate::domain::aggregate::Plane;
use crate::queries::current_journey::{CurrentJourneyQuery, CurrentJourneyView};
use crate::queries::logger::LoggingQuery;

pub fn plane_cqrs_framework(
    pool: Pool<Postgres>,
) -> (
    Arc<PostgresCqrs<Plane>>,
    Arc<PostgresViewRepository<CurrentJourneyView, Plane>>,
) {
    let logger_query = LoggingQuery {};
    let view_repo = Arc::new(PostgresViewRepository::new("plane_query", pool.clone()));
    let mut current_journey_query = CurrentJourneyQuery::new(view_repo.clone());

    // Without a query error handler there will be no indication if an
    // error occurs (e.g., database connection failure, missing columns or table).
    // Consider logging an error or panicking in your own application.
    current_journey_query.use_error_handler(Box::new(|e| println!("{}", e)));

    // Create and return an event-sourced `CqrsFramework`.
    let queries: Vec<Box<dyn Query<Plane>>> =
        vec![Box::new(logger_query), Box::new(current_journey_query)];
    let services = ();
    (
        Arc::new(postgres_es::postgres_cqrs(pool, queries, services)),
        view_repo,
    )
}
