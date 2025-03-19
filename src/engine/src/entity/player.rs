use crate::entity::block_walk::BlockWalk;
use crate::entity::entity::{EntityBehavior, PathingEntity};
use crate::entity::entity_lifecycle::EntityLifeCycle;
use crate::entity::move_restrict::MoveRestrict;
use crate::entity::move_strategy::MoveStrategy;
use crate::entity::window_status::WindowStatus;
use crate::grid::coord_grid::CoordGrid;
use constants::window_mode::window_mode;

pub struct LevelExperience {
    experience_table: [i32; 99],
}

impl LevelExperience {
    pub fn new() -> Self {
        let mut experience_table: [i32; 99] = [0; 99];
        let mut acc = 0;

        for i in 0..99 {
            let level = i as f64 + 1.0;
            let delta = (level + 2.0_f64.powf(level / 7.0) * 300.0).floor() as i32;
            acc += delta;
            experience_table[i] = (acc / 4) * 10;
        }
        
        Self { experience_table }
    }
    
    pub fn get_level_by_experience(&self, experience: i32) -> i32 {
        for i in (0..99).rev() {
            if experience >= self.experience_table[i] {
                return (i + 2).min(99) as i32;
            }
        }
        1
    }
    
    pub fn get_experience_by_level(&self, level: i32) -> i32 {
        if level < 2 || level > 100 {
            panic!("Level must be between 2 and 100");
        }
        self.experience_table[(level - 2) as usize]
    }
}

#[derive(Copy, Clone)]
pub struct Player {
    // Permanent
    pub pathing_entity: PathingEntity,
    pub move_restrict: MoveRestrict,
    pub block_walk: BlockWalk,
    pub move_strategy: MoveStrategy,
    pub gender: u8,
    pub playtime: i32,
    
    pid: usize,
    
    pub origin_coord: CoordGrid,
    
    // Client data
    pub window_status: WindowStatus,
    
    staff_mod_level: i32,
    
    pub request_logout: bool,
    pub request_idle_logout: bool,
    pub logging_out: bool,
    pub prevent_logout_until: i32,
    
    pub last_response: i32,
    pub last_connected: i32,
    pub verify_id: u32,
    // TODO - Active Script

}

impl Player {
    pub fn new(coord: CoordGrid, gender: u8, window_status: WindowStatus, staff_mod_level: i32, pid: usize) -> Player {
        Player {
            pathing_entity: PathingEntity::new(
                coord,
                1,
                1,
                EntityLifeCycle::FOREVER
            ),
            move_restrict: MoveRestrict::Normal,
            block_walk: BlockWalk::Npc,
            move_strategy: MoveStrategy::Smart,
            gender,
            playtime: -1,
            pid,
            origin_coord: CoordGrid { coord: 0 },
            staff_mod_level,
            window_status,
            request_logout: false,
            request_idle_logout: false,
            logging_out: false,
            prevent_logout_until: -1,
            last_response: -1,
            last_connected: -1,
            verify_id: 0,
        }
    }
    
    pub fn new_dummy(coord: CoordGrid, gender: u8, pid: usize) -> Player {
        Player {
            pathing_entity: PathingEntity::new(
              coord,
              1,
              1,
              EntityLifeCycle::FOREVER
            ),
            move_restrict: MoveRestrict::Normal,
            block_walk: BlockWalk::Npc,
            move_strategy: MoveStrategy::Smart,
            gender,
            playtime: -1,
            pid,
            origin_coord: CoordGrid { coord: 0 },
            staff_mod_level: 0,
            window_status: WindowStatus { window_mode: window_mode::NULL, canvas_width: 0, canvas_height: 0, anti_aliasing_mode: 0 },
            request_logout: false,
            request_idle_logout: false,
            logging_out: false,
            prevent_logout_until: -1,
            last_response: -1,
            last_connected: -1,
            verify_id: 0,
        }
    }
    
    pub(crate) fn get_coord(self) -> CoordGrid {
        self.pathing_entity.coord()
    }

    pub(crate) fn set_coord(&mut self, coord: CoordGrid) {
        self.pathing_entity.set_coord(coord);
    }

    pub(crate) fn get_origin_coord(self) -> CoordGrid {
        self.origin_coord
    }

    pub(crate) fn set_origin_coord(&mut self, coord: CoordGrid) {
        self.origin_coord = coord;
    }

    pub(crate) fn get_active(self) -> bool {
        self.pathing_entity.active()
    }
    
    pub(crate) fn set_active(&mut self, active: bool) {
        self.pathing_entity.set_active(active);
    }
    
    pub(crate) fn get_verify_id(self) -> u32 {
        self.verify_id
    }
    
    pub(crate) fn get_incremented_verify_id(&mut self) -> u32 {
        self.verify_id = self.verify_id +1;
        self.get_verify_id()
    }
    pub(crate) fn set_verify_id(&mut self, verify_id: u32) {
        self.verify_id = verify_id;
    }
    
    pub(crate) fn get_staff_mod_level(self) -> i32 {
        self.staff_mod_level
    }
    
    pub(crate) fn get_pid(self) -> usize {
        self.pid
    }
    
    pub(crate)fn set_pid(&mut self, pid: usize) {
        self.pid = pid;
    }
}