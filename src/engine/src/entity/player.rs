use constants::window_mode::window_mode;
use crate::entity::block_walk::BlockWalk;
use crate::entity::entity::Entity;
use crate::entity::entity_lifecycle::EntityLifeCycle;
use crate::entity::move_restrict::MoveRestrict;
use crate::entity::move_strategy::MoveStrategy;
use crate::entity::window_status::WindowStatus;
use crate::grid::coord_grid::CoordGrid;

#[derive(Copy, Clone)]
pub struct Player {
    // Permanent
    pub entity: Entity, // TODO - should be pathing entity.
    pub move_restrict: MoveRestrict,
    pub block_walk: BlockWalk,
    pub move_strategy: MoveStrategy,
    pub gender: u8,
    pub playtime: i32,
    
    pub pid: usize,
    
    pub origin_coord: CoordGrid,
    
    // Client data
    pub window_status: WindowStatus,
    
    pub request_logout: bool,
    pub request_idle_logout: bool,
    pub logging_out: bool,
    pub prevent_logout_until: i32,
    
    pub last_response: i32,
    pub last_connected: i32,
    // TODO - Active Script
}

impl Player {
    pub fn new(coord: CoordGrid, gender: u8, window_status: WindowStatus, pid: usize) -> Player {
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
            origin_coord: CoordGrid { coord: 0 },

            window_status,
            request_logout: false,
            request_idle_logout: false,
            logging_out: false,
            prevent_logout_until: -1,
            last_response: -1,
            last_connected: -1
        }
    }
    
    pub fn new_dummy(coord: CoordGrid, gender: u8, pid: usize) -> Player {
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
            origin_coord: CoordGrid { coord: 0 },
            window_status: WindowStatus { window_mode: window_mode::NULL, canvas_width: 0, canvas_height: 0, anti_aliasing_mode: 0 },
            request_logout: false,
            request_idle_logout: false,
            logging_out: false,
            prevent_logout_until: -1,
            last_response: -1,
            last_connected: -1
        }
    }
    
    pub(crate) fn get_coord(self) -> CoordGrid {
        self.entity.coord
    }

    pub(crate) fn set_coord(&mut self, coord: CoordGrid) {
        self.entity.coord = coord;
    }

    pub(crate) fn get_origin_coord(self) -> CoordGrid {
        self.origin_coord
    }

    pub(crate) fn set_origin_coord(&mut self, coord: CoordGrid) {
        self.origin_coord = coord;
    }

    pub(crate) fn get_active(self) -> bool {
        self.entity.active
    }
    
    pub(crate) fn set_active(&mut self, active: bool) {
        self.entity.active = active;
    }
}