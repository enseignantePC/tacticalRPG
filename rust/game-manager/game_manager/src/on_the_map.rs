//! This module is responsible of everything that can `physically` be on the map
//! - [Entity]s, that requires input to function
//! - [Object]s, that should be intractable by every entity AND, shouldn't be stepped on when walking on the map (TODO)
//! - [Obstacle]s, something intractable that doesn't require input : destructible walls, traps, etc


use std::{collections::HashMap, fmt::Debug};

use crate::{map::terrains::TerrainType, EntityId, TeamId};

pub mod entity;
pub use entity::*;

/// interactive object present on the map that any entity can interact with
/// (interrupter, usable etc, card to collection, ammo, new weapons)
/// ? should the logic be handled outside of the game via a kind of signal? sounds like a good idea
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Object {}

/// movable or destructable (or interactable?) obstacle on the map
///
/// destructable walls, traps
/// ? should the logic be handled outside of the game via a kind of signal? sounds like a good idea
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Obstacle {}
