// https://x.com/JagexAsh/status/1677654049238265857
#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum BlockWalk {
    None = 0,
    Npc = 1,
    All = 2,
}