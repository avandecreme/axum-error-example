use derive_more::Display;
use serde::Serialize;

use crate::business_logic::{self, JobId};

#[derive(Serialize)]
pub(crate) struct Job {
    pub id: JobId,
    pub description: String,
    pub state: JobState,
    pub result: Option<JobResult>,
}

impl From<crate::business_logic::Job> for Job {
    fn from(
        crate::business_logic::Job {
            id,
            description,
            state,
        }: crate::business_logic::Job,
    ) -> Self {
        Job {
            id,
            description,
            state: (&state).into(),
            result: state.into(),
        }
    }
}

#[derive(Debug, Display, Serialize)]
pub(crate) enum JobState {
    Created,
    Running,
    Done,
    Canceled,
}

impl From<&business_logic::JobState> for JobState {
    fn from(value: &business_logic::JobState) -> Self {
        match value {
            business_logic::JobState::Created => JobState::Created,
            business_logic::JobState::Running => JobState::Running,
            business_logic::JobState::Done(_) => JobState::Done,
            business_logic::JobState::Canceled => JobState::Canceled,
        }
    }
}

#[derive(Serialize)]
pub(crate) enum JobResult {
    Success { value: i32 },
    Failure { error_msg: String },
}

impl From<business_logic::JobState> for Option<JobResult> {
    fn from(value: business_logic::JobState) -> Self {
        match value {
            business_logic::JobState::Created
            | business_logic::JobState::Running
            | business_logic::JobState::Canceled => None,
            business_logic::JobState::Done(job_result) => Some(job_result.into()),
        }
    }
}

impl From<business_logic::JobResult> for JobResult {
    fn from(value: business_logic::JobResult) -> Self {
        match value {
            business_logic::JobResult::Success { value } => JobResult::Success { value },
            business_logic::JobResult::Failure { error_msg } => JobResult::Failure { error_msg },
        }
    }
}
