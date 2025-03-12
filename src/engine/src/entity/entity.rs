use crate::engine::Engine;
use crate::entity::entity_lifecycle::EntityLifeCycle;
use crate::grid::coord_grid::CoordGrid;

#[derive(Eq, Hash, PartialEq)]
pub struct Entity {
    pub coord: CoordGrid,
    pub width: u8,
    pub length: u8,
    pub lifecycle: EntityLifeCycle,
    pub active: bool,
    
    pub lifecycle_tick: u32,
    pub last_lifecycle_tick: u32,
}

impl Entity {

    pub fn new(coord: CoordGrid, width: u8, length: u8, lifecycle: EntityLifeCycle) -> Self {
        Entity {
            coord,
            width,
            length,
            lifecycle,
            lifecycle_tick: 0,
            last_lifecycle_tick: 0,
            active: false,
        }
    }
    
    /*pub fn update_lifecycle(&self, tick: u32) -> bool {
        //self.lifecycle_tick == tick..unwrap() && self.lifecycle == EntityLifeCycle::Forever
    }*/
    
    pub fn check_lifecycle(&self, tick: u32) -> bool {
        if self.lifecycle == EntityLifeCycle::Forever {
            return true
        }
        
        if self.lifecycle == EntityLifeCycle::Respawn { 
            return self.lifecycle_tick < tick;
        }
        
        if self.lifecycle == EntityLifeCycle::Despawn {
            return self.lifecycle_tick > tick;
        }
        
        false
    }
    
    pub fn set_lifecycle(&mut self, tick: u32) {
       // TODO
    }
    
}

pub trait PathingEntity{}

pub trait NonPathingEntity{}