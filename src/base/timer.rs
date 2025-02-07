use std::str::FromStr;

use chrono::Local;

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
