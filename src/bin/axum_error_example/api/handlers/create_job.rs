use axum::{Json, response::IntoResponse};
use derive_more::From;
use enum_convert::EnumInto;
use serde::Deserialize;

use crate::{
    api::{ApiError, types::Job},
    business_logic::{self, JobId, JobState},
    db::{self, DbError},
};

#[derive(Deserialize)]
pub struct CreateJobRequest {
    pub description: String,
}

#[derive(Debug, From, EnumInto)]
#[enum_into(ApiError)]
pub enum CreateJobError {
    #[from(anyhow::Error, DbError)]
    InternalError(anyhow::Error),
}

impl IntoResponse for CreateJobError {
    fn into_response(self) -> axum::response::Response {
        ApiError::from(self).into_response()
    }
}

#[axum::debug_handler]
pub async fn create_job(
    Json(CreateJobRequest { description }): Json<CreateJobRequest>,
) -> Result<Json<Job>, CreateJobError> {
    let job = business_logic::Job {
        id: JobId::new(),
        description,
        state: JobState::Created,
    };
    let job = db::insert_job(job).await?;

    Ok(Json(job.into()))
}
