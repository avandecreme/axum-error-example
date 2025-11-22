use axum::{Json, response::IntoResponse};
use derive_more::From;
use enum_convert::EnumInto;

use crate::{
    api::{ApiError, types::Job},
    db::{self, DbError},
};

#[derive(Debug, From, EnumInto)]
#[enum_into(ApiError)]
pub enum ListJobsError {
    #[from(anyhow::Error, DbError)]
    InternalError(anyhow::Error),
}

impl IntoResponse for ListJobsError {
    fn into_response(self) -> axum::response::Response {
        ApiError::from(self).into_response()
    }
}

#[axum::debug_handler]
pub async fn list_jobs() -> Result<Json<Vec<Job>>, ListJobsError> {
    let jobs = db::get_jobs().await?;
    Ok(Json(jobs.into_iter().map(Into::into).collect()))
}
