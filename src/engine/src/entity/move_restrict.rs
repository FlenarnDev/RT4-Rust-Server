#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum MoveRestrict {
    Normal = 0,
    Blocked = 1,
    BlockedNormal = 2,
    Indoors = 3,
    Outdoors = 4,
    NoMove = 5,
    PassThru = 6,
}