use crate::entity::block_walk::BlockWalk;
use crate::entity::entity::Entity;
use crate::entity::entity_lifecycle::EntityLifeCycle;
use crate::entity::move_restrict::MoveRestrict;
use crate::entity::move_strategy::MoveStrategy;
use crate::grid::coord_grid::CoordGrid;

pub struct Player {
    // Permanent
    pub entity: Entity, // TODO - should be pathing entity.
    pub move_restrict: MoveRestrict,
    pub block_walk: BlockWalk,
    pub move_strategy: MoveStrategy,
    pub gender: u8,
    pub playtime: i32,
    
    pub pid: i32,
    
    pub request_logout: bool,
    pub request_idle_logout: bool,
    pub logging_out: bool,
    pub prevent_logout_until: i32,
    
    pub last_response: i32,
    pub last_connected: i32,
    // TODO - Active Script
}

impl Player {
    pub fn new(coord: CoordGrid, gender: u8, pid: i32) -> Player {
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
            playtime: -1,
            pid,
            request_logout: false,
            request_idle_logout: false,
            logging_out: false,
            prevent_logout_until: -1,
            last_response: -1,
            last_connected: -1
        }
    }
}

