use crate::entity::entity::Entity;
use crate::entity::entity_lifecycle::EntityLifeCycle;
use crate::grid::coord_grid::CoordGrid;

#[derive(Copy, Clone, PartialEq)]
pub struct PathingEntity {
    pub entity: Entity,
    pub(crate) delayed: bool,
    pub(crate) delayed_until: i32,
}

impl PathingEntity {
    pub fn new(coord: CoordGrid, width: u8, length: u8, lifecycle: EntityLifeCycle) -> Self {
        PathingEntity {
            entity: Entity::new(coord, width, length, lifecycle),
            delayed: false,
            delayed_until: -1,
        }
    }
}