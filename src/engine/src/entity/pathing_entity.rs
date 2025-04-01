use crate::entity::block_walk::BlockWalk;
use crate::entity::entity::Entity;
use crate::entity::entity_lifecycle::EntityLifeCycle;
use crate::entity::move_restrict::MoveRestrict;
use crate::entity::move_speed::MoveSpeed;
use crate::entity::move_strategy::MoveStrategy;
use crate::entity::npc_mode::NpcMode;
use crate::grid::coord_grid::CoordGrid;
use crate::script::server_trigger_types::ServerTriggerTypes;

pub struct TargetSubject {
    pub type_: u32,
    pub com: u32,
}

pub enum TargetOp {
    Trigger(ServerTriggerTypes),
    Mode(NpcMode)
}

#[derive(Copy, Clone, PartialEq)]
pub struct PathingEntity {
    // Constructor properties
    pub entity: Entity,
    //pub move_restrict: MoveRestrict,
    //pub block_walk: BlockWalk,
    //pub move_strategy: MoveStrategy,
    //pub coord_mask: u32,
    //pub entity_mask: u32,
    
    // Runtime properties
    move_speed: MoveSpeed,
    pub(crate) delayed: bool,
    pub(crate) delayed_until: i32,
}

impl PathingEntity {
    pub fn new(coord: CoordGrid, width: u8, length: u8, lifecycle: EntityLifeCycle) -> Self {
        PathingEntity {
            entity: Entity::new(coord, width, length, lifecycle),
            move_speed: MoveSpeed::INSTANT,
            delayed: false,
            delayed_until: -1,
        }
    }
}