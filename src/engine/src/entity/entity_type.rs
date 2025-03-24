use crate::entity::loc::Loc;
use crate::entity::npc::NPC;
use crate::entity::obj::Obj;
use crate::entity::player::Player;

#[derive(Clone, PartialEq)]
pub enum EntityType {
    Player(Player),
    NPC(NPC),
    Loc(Loc),
    Obj(Obj)
}