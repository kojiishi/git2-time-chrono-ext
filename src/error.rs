#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid TimeZone {offset_minutes}")]
    InvalidTimeZone { offset_minutes: i32 },
    #[error("Time {time:?} isn't mappable")]
    TimeNotMappable { time: git2::Time },
}
