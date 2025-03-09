use crate::entity::block_walk::BlockWalk;
use crate::entity::entity::Entity;
use crate::entity::entity_lifecycle::EntityLifeCycle;
use crate::entity::move_restrict::MoveRestrict;
use crate::entity::move_strategy::MoveStrategy;
use crate::grid::coord_grid::CoordGrid;

pub struct Player {
    // Permanent
    pub entity: Entity,
    pub move_restrict: MoveRestrict,
    pub block_walk: BlockWalk,
    pub move_strategy: MoveStrategy,
    pub gender: u8,
    pub playtime: i32,
    
    // Temporary
    pub uid: i32,
    pub mask: i32,
    pub anim_id: i32,
    pub anim_delay: i32,
    pub anim_protect: bool,
    pub bas_readyanim: i32,
    // TODO - Active Script
}

impl Player {
    pub fn new(coord: CoordGrid, gender: u8) -> Player {
        Player {
            entity: Entity::new(
              coord,
              1,
              1,
              EntityLifeCycle::Forever
            ),
            move_restrict: MoveRestrict::Normal,
            block_walk: BlockWalk::Npc,
            move_strategy: MoveStrategy::Smart,
            gender,
            uid: -1,
            mask: 0,
            anim_id: -1,
            anim_delay: 0,
            anim_protect: false,
            bas_readyanim: -1,
            playtime: -1,
        }
    }
}