//! This module is responsible of everything that can `physically` be on the map
//! - [Entity]s, that requires input to function
//! - [Object]s, that should be interactable by every entity AND, shouldn't be stepped on when walking on the map (TODO)
//! - [Obstacle]s, something interactable that doesnt require input : destructable walls, traps, etc
//!

use std::collections::HashMap;

///
use super::{map, EntityId, TeamID};

/// An entity is the type of thing on the map that requires input to decide what to do.
/// You should think of entities as the only "alive" thing on the map
///
/// As entities are the most important thing on the map
/// they should be the most flexible thing possible
///
/// It makes more and more sense that it should be a trait
/// and maybe the entity should be handled entirerly by the external source,
/// possibly in the form of a Godot Node, capable of calling some gdscript code to satisfy the trait
/// (which would probably make it unsafe)?
///
/// As i am too much of a newbie at rust, this frigthens me however
#[derive(Debug)]
pub struct Entity {
    pub team: TeamID,
    pub unique_id: EntityId,
    pub terrain_weights: HashMap<map::terrains::TerrainType, f32>,
}

/// interactive object present on the map that any enity can interact with
/// (interruptor, usable etc, card to collection, ammo, new weapons)
/// ? should the logic be handled outside of the game via a kind of signal? sounds like a good idea
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Object {}

/// movable or destructable (or interactable?) obstacle on the map
///
/// destructable walls, traps
/// ? should the logic be handled outside of the game via a kind of signal? sounds like a good idea
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Obstacle {}

impl Entity {
    /// creates a entity for test purposes
    pub fn example_entity() -> Entity {
        let mut h: HashMap<map::terrains::TerrainType, f32> = HashMap::new();
        h.insert(map::terrains::TerrainType::Ground, 1f32);
        h.insert(map::terrains::TerrainType::Forest, 4f32);
        Entity {
            team: TeamID::Loner,
            terrain_weights: h,
            unique_id: EntityId(0),
        }
    }

    // TODO : properly implement this
    /// determines how far the entity will be able to move
    pub fn get_move_force(&self) -> f32 {
        10.0
    }

    // TODO : properly implement this
    #[deprecated = "prout"]
    /// damage reduction when fighting, should depend on SOMETHING
    pub fn damage_reduction_factor(&self) -> f64 {
        1f64
    }

    // TODO : properly implement this
    #[deprecated = "prout"]
    /// At what distance(s) the entity can strike
    /// currently broken but should depend on
    /// - the weapon
    /// - the entity
    /// - ? a plethora of other stuff, should the logic be handled by the external source?
    pub(crate) fn get_attack_ranges(&self) -> &[i32] {
        &[1, 2]
    }
}
