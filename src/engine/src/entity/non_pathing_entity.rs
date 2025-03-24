use crate::entity::entity::Entity;
use crate::entity::entity_lifecycle::EntityLifeCycle;
use crate::grid::coord_grid::CoordGrid;

#[derive(Copy, Clone, PartialEq)]
pub struct NonPathingEntity {
    pub entity: Entity,
}

impl NonPathingEntity {
    pub fn new(coord: CoordGrid, width: u8, length: u8, lifecycle: EntityLifeCycle) -> Self {
        NonPathingEntity {
            entity: Entity::new(coord, width, length, lifecycle),
        }
    }
}