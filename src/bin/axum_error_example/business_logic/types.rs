use derive_more::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub(crate) struct Job {
    pub id: JobId,
    pub description: String,
    pub state: JobState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Serialize, Deserialize)]
pub(crate) struct JobId(Uuid);

impl JobId {
    pub fn new() -> JobId {
        JobId(Uuid::now_v7())
    }
}

#[derive(Debug, Clone)]
pub(crate) enum JobState {
    Created,
    Running,
    Done(JobResult),
    Canceled,
}

#[derive(Debug, Clone)]
pub(crate) enum JobResult {
    Success { value: i32 },
    Failure { error_msg: String },
}
