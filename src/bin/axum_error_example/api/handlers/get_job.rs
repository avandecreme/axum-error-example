use axum::{Json, extract::Path, response::IntoResponse};
use derive_more::From;
use enum_convert::EnumInto;

use crate::{
    api::{ApiError, types::Job},
    business_logic::JobId,
    db::{self, DbError},
};

#[derive(Debug, From, EnumInto)]
#[enum_into(ApiError)]
pub enum GetJobError {
    #[from(anyhow::Error, DbError)]
    InternalError(anyhow::Error),

    JobNotFound(JobId),
}

impl IntoResponse for GetJobError {
    fn into_response(self) -> axum::response::Response {
        ApiError::from(self).into_response()
    }
}

#[axum::debug_handler]
pub async fn get_job(Path(job_id): Path<JobId>) -> Result<Json<Job>, GetJobError> {
    let job = db::get_job(job_id)
        .await?
        .ok_or_else(|| GetJobError::JobNotFound(job_id))?;

    Ok(Json(job.into()))
}
