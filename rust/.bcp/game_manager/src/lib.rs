#![allow(
    dead_code,
    unused_imports
)]

//! # The Game Manager crate
//!
//! It builds on the dijkstra-map crate to provide a powerful system
//! and expressive system for dealing with anything tactical rpg related
//! (or at least it's is goal).
//!
//! It's being designed with the objectives of being
//! - complete
//! - extendable
//! - customizable
//!
//! In particular it should be easy to plug in your own logic
//! if you want to override what defines an Actor, how the combat system works
//! in general etc..
//!
//! It's full workflow should be explained with beautiful diagrams and such
//! but we're not there yet!
//!
//! note : add links up there ;)
//!
//! Features :
//! - [ ] complete this list ;)
//! - [ ] Override logic systems from gdscript

/// computes map pathfinding calculations
use dijkstra_map::DijkstraMap;
pub use gdnative;
/// godot side wrappers
use gdnative::prelude::*;
/// TODO : Documentation
/// how to get valid inputs from the lib
/// how to select them out of the lib and then inform the lib
///
/// TODO : feature
/// design an entity that leaves a trail of something
/// - entity ally go faster in the trail?
/// - entity does more damage if in own trail?
///
/// a system of optional tags on the map that are used by entity to determine stuff
use std::collections::HashMap;
pub use thiserror;
/// This module is responsible for offering a description of things
/// that entity can do while the game is playing
/// such things are called actions and represented by an [Action] structure.
pub mod entity_action;
pub use entity_action::*;
/// This module is responsible for Turning an [Attack]
/// (a description of an attack with an uncertain result) into
/// an [attack_solver::ResolvedAttack] (a result without any randomness involved).
pub mod attack_solver;
/// This module is responsible for representing the world
/// as a 2D grid and computing/keeping track of everything that happens,
/// grid wise.
/// Computation are made using an intern [DijkstraMap].
pub mod map;

/// This module is responsible for everything that is interactive
/// and on the map, entities, object, destructible terrains.
pub mod on_the_map;
pub use on_the_map::Entity;
/// Exposes an [Intent] struct that means what an Entity would like to do if possible.
/// Intents can be emitted and subscribed to the game manager at some points of the execution.
/// They must be analysed and judged still possible to be transformed into a [WorldChange]
/// and be used to update the world state.
pub mod turn_logic;
pub use turn_logic::Intent;
pub use turn_logic::*;

pub mod world_manager;
pub use world_manager::*;

/// expose a [Watcher] structure, which is used to implement how an entity will react to something
/// (by wanting to counter attack for instance).
///
/// It will analyse incoming [Intents][Intent] and
/// emit [Intent]s as a response.
///
/// WARNING : side effect here
pub mod watcher;
use watcher::*;

/// main interfaces that glue modules together
pub mod game_manager_mod;
pub use game_manager_mod::*;

pub enum GameStatus {
    FightNotStarted,
    EntityWaitingForInput(EntityId),
    FightEnded,
}
