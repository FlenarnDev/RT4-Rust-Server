#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum MoveSpeed {
    STATIONARY,
    CRAWL,
    WALK,
    RUN,
    INSTANT
}