use crate::entity::block_walk::BlockWalk;
use crate::entity::entity::Entity;
use crate::entity::entity_lifecycle::EntityLifeCycle;
use crate::entity::entity_type::EntityType;
use crate::entity::move_restrict::MoveRestrict;
use crate::entity::move_strategy::MoveStrategy;
use crate::entity::pathing_entity::PathingEntity;
use crate::grid::coord_grid::CoordGrid;

#[derive(Clone, PartialEq)]
pub struct NPC {
    pub pathing_entity: PathingEntity,
    pub move_restrict: MoveRestrict,
    pub block_walk: BlockWalk,
    pub move_strategy: MoveStrategy,
    pub nid: i32,
    pub id: u16, // Cache 'ID'
}

impl NPC {
    pub fn new(coord: CoordGrid, width: u8, length: u8, lifecycle: EntityLifeCycle, nid: i32, id: u16, move_restrict: MoveRestrict, block_walk: BlockWalk) -> NPC {
        NPC {
            pathing_entity: PathingEntity::new(
                coord,
                width,
                length,
                lifecycle,
            ),
            move_restrict,
            block_walk,
            move_strategy: MoveStrategy::Naive,
            nid,
            id,
        }
    }
    
    pub fn get_entity(&self) -> &Entity {
        &self.pathing_entity.entity
    }
    
    pub fn into_entity_type(self) -> EntityType {
        EntityType::NPC(self)
    }
}