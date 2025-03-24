#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum MoveStrategy {
    Smart = 0,
    Naive = 1,
    Fly = 2,
}