use std::{collections::HashMap, sync::LazyLock};

use tokio::sync::RwLock;

use crate::business_logic::{Job, JobId, JobState};

static JOBS: LazyLock<RwLock<HashMap<JobId, Job>>> = LazyLock::new(|| RwLock::new(HashMap::new()));

#[derive(Debug, thiserror::Error)]
pub(crate) enum DbError {
    #[error("connection error")]
    #[expect(unused)]
    ConnectionError,
    #[error("table '{0}' not found")]
    #[expect(unused)]
    TableNotFound(&'static str),
    #[error("row conflict: {0}")]
    RowConflict(String),
    #[error("row not found: {0}")]
    RowNotFound(String),
}

pub(crate) async fn insert_job(job: Job) -> Result<Job, DbError> {
    let mut jobs = JOBS.write().await;

    if jobs.contains_key(&job.id) {
        return Err(DbError::RowConflict(format!(
            "a row with primary key '{}' already exists in the jobs table",
            job.id
        )));
    }

    jobs.insert(job.id, job.clone());

    Ok(job)
}

pub(crate) async fn get_job(job_id: JobId) -> Result<Option<Job>, DbError> {
    let jobs = JOBS.read().await;
    Ok(jobs.get(&job_id).cloned())
}

pub(crate) async fn get_jobs() -> Result<Vec<Job>, DbError> {
    let jobs = JOBS.read().await;
    Ok(jobs.values().cloned().collect())
}

pub(crate) async fn get_next_job_to_run() -> Result<Option<Job>, DbError> {
    let mut jobs = JOBS.write().await;
    let mut job = jobs
        .values_mut()
        .find(|job| matches!(job.state, JobState::Created));
    if let Some(job) = &mut job {
        job.state = JobState::Running;
    }
    Ok(job.cloned())
}

pub(crate) async fn update_job_state(job_id: JobId, job_state: JobState) -> Result<Job, DbError> {
    let mut jobs = JOBS.write().await;
    if let Some(job) = jobs.get_mut(&job_id) {
        job.state = job_state;
        Ok(job.clone())
    } else {
        Err(DbError::RowNotFound(format!(
            "no row with job_id '{job_id}' found on jobs table"
        )))
    }
}
