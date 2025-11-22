use axum::{Json, extract::Path, response::IntoResponse};
use derive_more::From;
use enum_convert::EnumInto;

use crate::{
    api::{
        ApiError,
        types::{Job, JobState},
    },
    business_logic::{self, JobId},
    db::{self, DbError},
};

#[derive(Debug, From, EnumInto)]
#[enum_into(ApiError)]
pub enum ResetJobError {
    #[from(anyhow::Error, DbError)]
    InternalError(anyhow::Error),

    JobNotFound(JobId),

    UnexpectedJobState {
        actual: JobState,
        expected: Vec<JobState>,
    },
}

impl IntoResponse for ResetJobError {
    fn into_response(self) -> axum::response::Response {
        ApiError::from(self).into_response()
    }
}

#[axum::debug_handler]
pub async fn reset_job(Path(job_id): Path<JobId>) -> Result<Json<Job>, ResetJobError> {
    let job = db::get_job(job_id)
        .await?
        .ok_or_else(|| ResetJobError::JobNotFound(job_id))?;

    let job = match job.state {
        business_logic::JobState::Created | business_logic::JobState::Running => {
            return Err(ResetJobError::UnexpectedJobState {
                actual: (&job.state).into(),
                expected: vec![JobState::Done, JobState::Canceled],
            });
        }
        business_logic::JobState::Done(_) | business_logic::JobState::Canceled => {
            db::update_job_state(job_id, business_logic::JobState::Created).await?
        }
    };

    Ok(Json(job.into()))
}
