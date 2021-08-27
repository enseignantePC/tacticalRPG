use std::collections::HashMap;

use crate::{map, EntityId};

/// everything that is on the map
///
use super::TeamID;

/// An entity is the type of thing on the map that requires input to decide what to do.
/// You should think of entities as the only "alive" thing on the map
#[derive(Debug)]
pub struct Entity {
    pub team: TeamID,
    pub unique_id: EntityId,
    pub terrain_weights: HashMap<map::terrains::TerrainType, i32>,
}

/// interactive object present on the map (interruptor, usable etc)
pub struct Object {}
/// movable or destructable (or interactable?) obstacle on the map
pub struct Obstacle {}

impl Entity {
    /// creates a entity for test purposes
    pub fn example_entity() -> Entity {
        let mut h: HashMap<map::terrains::TerrainType, i32> = HashMap::new();
        h.insert(map::terrains::TerrainType::Ground, 1);
        h.insert(map::terrains::TerrainType::Forest, 4);
        Entity {
            team: TeamID::Loner,
            terrain_weights: h,
            unique_id: EntityId(0),
        }
    }

    // TODO : properly implement this
    pub fn get_move_force(&self) -> f32 {
        10.0
    }

    // TODO : properly implement this
    #[deprecated = "prout"]
    pub fn damage_reduction_factor(&self) -> f64 {
        1f64
    }

    pub(crate) fn get_attack_ranges(&self) -> &[f32] {
        todo!()
    }
}
