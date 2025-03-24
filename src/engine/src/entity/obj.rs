use crate::entity::entity::Entity;
use crate::entity::entity_lifecycle::EntityLifeCycle;
use crate::grid::coord_grid::CoordGrid;

#[derive(Clone, PartialEq)]
pub struct Obj {
    pub entity: Entity,
    pub id: u16,
    pub count: u32,
    pub receiver_id: i32,
    pub reveal: i32,
    pub last_change: i32,
}

impl Obj {
    /// The number of ticks for an obj to reveal.
    pub const REVEAL: u8 = 100;

    pub fn new(coord: CoordGrid, lifecycle: EntityLifeCycle, id: u16, count: u32) -> Obj {
        Obj {
            entity: Entity::new(
                coord,
                1,
                1,
                lifecycle,
            ),
            id,
            count,
            receiver_id: -1,
            reveal: -1,
            last_change: -1,
        }
    }
}