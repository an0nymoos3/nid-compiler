use std::time::{Duration, Instant};

/// Returns current time as Instant
pub fn time_now() -> Instant {
    Instant::now()
}

/// Calculates time elapsed since parameter start_time
pub fn calc_total_time(start_time: &Instant) -> Duration {
    let now: Instant = Instant::now();
    now.duration_since(*start_time)
}
