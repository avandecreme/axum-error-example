use axum::{
    Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};

use crate::{
    api::{
        handlers::{cancel_job, create_job, get_job, list_jobs, reset_job},
        types::JobState,
    },
    business_logic::JobId,
};

mod handlers;
mod types;

pub fn router() -> Router {
    Router::new()
        .route("/jobs", post(create_job))
        .route("/jobs", get(list_jobs))
        .route("/jobs/{job_id}", get(get_job))
        .route("/jobs/{job_id}/reset", post(reset_job))
        .route("/jobs/{job_id}/cancel", post(cancel_job))
}

#[derive(Debug, thiserror::Error)]
enum ApiError {
    #[error(transparent)]
    InternalError(anyhow::Error),

    #[error("Job '{0}' not found")]
    JobNotFound(JobId),

    #[error(
        "Unexpected job state '{actual}', expected one of {}",
        expected.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", "))
    ]
    UnexpectedJobState {
        actual: JobState,
        expected: Vec<JobState>,
    },
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match &self {
            ApiError::InternalError(error) => {
                log::error!("{error:#}");
                // We don't return the error detail to not leak any info to the user
                (StatusCode::INTERNAL_SERVER_ERROR, "".to_owned())
            }
            ApiError::JobNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            ApiError::UnexpectedJobState { .. } => (StatusCode::BAD_REQUEST, self.to_string()),
        }
        .into_response()
    }
}
