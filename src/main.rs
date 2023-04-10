#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use rust_auto_sign_network_flow::{check_in::check_in, init, login::login};
use std::{error::Error, thread};
use tokio_scheduler_rs::{job_scheduler::JobScheduler, ScheduleJob};

struct CheckInJob;
impl ScheduleJob for CheckInJob {
    fn get_job_name(&self) -> String {
        String::from("AUTO CHECK IN")
    }

    fn execute(
        &self,
        id: String,
        args: Option<mysql::serde_json::Value>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        Box::pin(async {
            login().await.unwrap();
            check_in().await.unwrap();
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init()?;

    schedule_trigger().await;

    Ok(())
}

async fn schedule_trigger() {
    let scheduler = JobScheduler::default_with_timezone(chrono_tz::PRC);
    scheduler.register_job(Box::new(CheckInJob)).await.unwrap();
    scheduler
        .add_job("AUTO CHECK IN".into(), "0 30 9 * * * *".into(), None)
        .await
        .unwrap();
    scheduler.start().await.unwrap();
}

#[test]
fn feature() {}
