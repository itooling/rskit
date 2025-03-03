use std::str::FromStr;

use chrono::{DateTime, Local};

pub async fn task_cron<F>(cron: &str, f: F)
where
    F: Fn(),
    F: Send + 'static,
{
    if let Ok(schedule) = cron::Schedule::from_str(cron) {
        for interval in schedule.upcoming(Local) {
            if let Ok(duration) = interval.signed_duration_since(Local::now()).to_std() {
                tokio::time::sleep(duration).await;
                f();
            }
        }
    }
}
pub async fn task_cron_interval<F>(cron: &str, f: F)
where
    F: Fn(DateTime<Local>),
    F: Send + 'static,
{
    if let Ok(schedule) = cron::Schedule::from_str(cron) {
        for interval in schedule.upcoming(Local) {
            if let Ok(duration) = interval.signed_duration_since(Local::now()).to_std() {
                tokio::time::sleep(duration).await;
                f(interval);
            }
        }
    }
}

pub async fn task_interval<F>(period: std::time::Duration, f: F)
where
    F: Fn(),
    F: Send + 'static,
{
    let mut interval = tokio::time::interval(period);
    loop {
        interval.tick().await;
        f();
    }
}

pub async fn task_interval_args<F, A>(period: std::time::Duration, f: F, arg: A)
where
    F: Fn(&A),
    F: 'static + Send,
{
    let mut interval = tokio::time::interval(period);
    loop {
        interval.tick().await;
        f(&arg);
    }
}

pub async fn task_interval_at<F>(start: tokio::time::Instant, period: std::time::Duration, f: F)
where
    F: Fn(),
    F: Send + 'static,
{
    let mut interval = tokio::time::interval_at(start, period);
    loop {
        interval.tick().await;
        f();
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test(flavor = "multi_thread")]
    async fn test_task_cron_interval() {
        //use super::*;
        //task_cron_interval("0/5 * * * * *", |dt| println!("task: {:?}", dt)).await;
    }
}
