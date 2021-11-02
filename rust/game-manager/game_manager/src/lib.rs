#![allow(
    dead_code,
    unused_imports
)]
/// computes map pathfinding calculations
use dijkstra_map::DijkstraMap;
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
pub mod action;
pub use action::*;
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::on_the_map::Entity;

//     fn basic_initialise_map() -> map::Map {
//         map::Map::new(20, 20)
//     }
//     fn basic_initialise_game_manager() -> GameManager {
//         let manager = GameManager::new();
//         manager.initialise()
//     }
//     #[test]
//     fn basic_test() {
//         // initialize the game manager
//         let mut gm = basic_initialise_game_manager();
//         //  one player
//         let result = gm.register_entity(Entity::example_entity(), &map::Pos2D::new(0, 0));
//         result.unwrap();
//         panic!()
//     }
//     #[test]
//     fn can_retrieve_choices_from_game_manager() {
//         todo!()
//     }
//     #[test]
//     fn _move() {
//         // initialize the game manager
//         // on player turn make it move somewhere
//         // assert the player did move
//         todo!()
//     }
//     #[test]
//     fn _attack() {
//         // initialize the game manager
//         // add a second player
//         // on player turn make it attack somewhere
//         // assert the second player was damaged
//         todo!()
//     }
//     /// player A attack player B, player B counterAttacks
//     #[test]
//     fn _counter_attack() {
//         todo!()
//     }
//     /// player A attacks player B, a second attack will kill player B so player A attacks a second time
//     #[test]
//     fn double_attack_if_deadly() {
//         todo!()
//     }
//     /// player A attacks player B, a second attack wont kill player B so player A doesnt attacks a second time
//     #[test]
//     fn no_double_attack_if_not_deadly() {
//         todo!()
//     }
//     #[cfg(test)]
//     mod can_attack_if {
//         use super::*;

//         #[test]
//         fn in_range() {
//             todo!()
//         }
//         #[test]
//         fn not_if_not_in_range() {
//             todo!()
//         }
//         #[test]
//         fn loner_vs_loner() {
//             todo!()
//         }
//         #[test]
//         fn loner_vs_any_team() {
//             todo!()
//         }
//         #[test]
//         fn any_team_vs_loner() {
//             todo!()
//         }
//         #[test]
//         fn cant_if_same_team() {
//             todo!()
//         }
//         #[test]
//         fn different_teams() {
//             todo!()
//         }
//     }
// }
