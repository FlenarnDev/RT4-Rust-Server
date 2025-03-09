use crate::entity::entity::Entity;
use crate::entity::entity_lifecycle::EntityLifeCycle;
use crate::grid::coord_grid::CoordGrid;

#[derive(Eq, Hash, PartialEq)]
pub struct Loc {
    pub entity: Entity,
    pub info: u32,
}

impl Loc {
    pub fn new(
        coord: CoordGrid,
        width: u8,
        length: u8,
        lifecycle: EntityLifeCycle,
        id: u16,
        shape: u8,
        angle: u8
    ) -> Loc {
        Loc {
            entity: Entity::new(
                coord,
                width,
                length,
                lifecycle,
            ),
            info: ((id & 0x3fff) as u32)
                | (((shape & 0x1f) as u32) << 14)
                | (((angle & 0x1f) as u32) << 19)
        }
    }
    
    pub fn id(&self) -> u16 {
        (self.info & 0x3ff) as u16
    }
    
    pub fn shape(&self) -> u8 {
        ((self.info >> 14) & 0x1f) as u8
    }
    
    pub fn angle(&self) -> u8 {
        ((self.info >> 19) & 0x3) as u8
    }
}