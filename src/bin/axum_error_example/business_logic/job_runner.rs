use std::time::Duration;

use log::error;
use tokio::{task, time::sleep};

use crate::{
    business_logic::{Job, JobResult, JobState},
    db,
};

pub async fn job_runner() {
    loop {
        match db::get_next_job_to_run().await {
            Ok(Some(job)) => process_job(job).await,
            Ok(None) => {
                sleep(Duration::from_secs(1)).await;
            }
            Err(err) => {
                error!("Error getting next job to run: {err}");
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

async fn process_job(job: Job) {
    // Doing expensive computation
    let result = task::spawn_blocking(|| {
        // Fake expensive computation
        std::thread::sleep(Duration::from_secs(10));
        if rand::random_bool(0.9) {
            JobResult::Success {
                value: rand::random(),
            }
        } else {
            JobResult::Failure {
                error_msg: "Got unlucky".to_owned(),
            }
        }
    })
    .await;
    let result = match result {
        Ok(result) => result,
        Err(err) => JobResult::Failure {
            error_msg: format!("Error while executing job '{}': {err}", job.id),
        },
    };
    if let Err(err) = db::update_job_state(job.id, JobState::Done(result)).await {
        error!("Could not save result of job '{}': {err}", job.id);
    }
}
