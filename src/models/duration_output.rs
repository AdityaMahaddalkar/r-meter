use std::time::Duration;

#[derive(Debug)]
pub struct BasicDuration {
    pub(crate) time_to_execute: Duration,
}

#[derive(Debug)]
pub struct AggregateDuration {
    pub(crate) min_time_to_execute: Duration,
    pub(crate) max_time_to_execute: Duration,
    pub(crate) mean_time_to_execute: Duration,
}