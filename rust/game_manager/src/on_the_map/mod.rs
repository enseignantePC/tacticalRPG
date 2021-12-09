//! This module is responsible of everything that can `physically` be on the map
//! - [Entity]s, that requires input to function
//! - [Object]s, that should be intractable by every entity AND, shouldn't be stepped on when walking on the map (TODO)
//! - [crate::on_the_map::Obstacle]s, something intractable that doesn't require input : destructible walls, traps, etc

use std::{collections::HashMap, fmt::Debug, rc::Rc};

use crate::{map::terrains::Terrain, EntityId, TeamId};

pub mod entity;
pub use entity::*;

/// interactive object present on the map that any entity can interact with
/// (interrupter, usable etc, card to collection, ammo, new weapons)
/// ? should the logic be handled outside of the game via a kind of signal? sounds like a good idea
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Object {}

/// movable or destructible (or intractable?) obstacle on the map
///
/// destructible walls, traps
/// ? should the logic be handled outside of the game via a kind of signal? sounds like a good idea
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Obstacle {}

#[derive(Debug)]
/// everything interactive that can be in the world and will be stored by the map
pub enum Occupant {
    Entity(Rc<Entity>),
    Obstacle(Obstacle),
    Object(Object),
}

pub enum OccupantMask {
    AllExcept(Vec<OccupantMask>),
    Entity,
    Obstacle,
    Object,
}
impl OccupantMask {
    /// returns true if the occupant matches the occupant mask
    ///
    /// - if is entity and OccupantMask::Entity
    /// - if is object and OccupantMask::Object
    /// - if is obstacle and OccupantMask::Obstacle
    /// - if is any and not in except vector
    pub fn select(
        &self,
        occupant: &Occupant,
    ) -> bool {
        match self {
            OccupantMask::AllExcept(v) => {
                for occupant_mask in v.iter() {
                    if let OccupantMask::AllExcept(_occupant_masks) = occupant_mask {
                        panic!("Nested OccupantMask::AllExcept dont have meaning")
                    };
                }
                !v.iter().any(|x| x.select(occupant))
            }
            OccupantMask::Entity => {
                matches!(
                    occupant,
                    Occupant::Entity(_x)
                )
            }
            OccupantMask::Obstacle => {
                matches!(
                    occupant,
                    Occupant::Obstacle(_x)
                )
            }
            OccupantMask::Object => {
                matches!(
                    occupant,
                    Occupant::Object(_x)
                )
            }
        }
    }
}

/// Describes which Teams should or should not
/// match while doing a query.
///
///
pub enum TeamMask {
    AllExcept(Vec<TeamMask>),
    ThisTeam(TeamId),
    NotThisTeam(TeamId),
}

impl TeamMask {
    pub fn select(
        &self,
        id: TeamId,
    ) -> bool {
        match self {
            TeamMask::AllExcept(v) => v.iter().any(|x| {
                if let TeamMask::AllExcept(_team_masks) = x {
                    panic!("Nested TeamMask::AllExcept dont have meaning")
                };
                !v.iter().any(|x| -> bool { x.select(id) })
            }),
            TeamMask::ThisTeam(other) => *other == id,
            TeamMask::NotThisTeam(other) => *other != id,
        }
    }
}
