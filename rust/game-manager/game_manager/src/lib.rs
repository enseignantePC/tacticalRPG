#![allow(dead_code, unused_imports)]
/// computes map calculations
use dijkstra_map::DijkstraMap;
use gdnative::prelude::*;
use on_the_map::Entity;
/// TODO : Documentation
/// how to get valid inputs from the lib
/// how to select them out of the lib and then inform the lib
///
use std::collections::HashMap;

pub mod action;
use action::*;

pub mod attack_solver;
pub mod map;

pub mod on_the_map;

pub mod turn_logic;
use turn_logic::*;

pub mod world_manager;
use world_manager::*;

/// expose a [Watcher] structure, responsible of analysing incoming intents or [WorldChange]s and
/// yields [Intent]s as a response
pub mod watcher;
use watcher::*;

/// expose a structure responsible for communicating with an external sources that will provide inputs
pub mod input_manager;
use input_manager::*;

/// main interfaces that glue modules together
pub mod game_manager;
pub use game_manager::*;

pub enum Status {
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
//     fn can_retrieve_choices_from_gamemanager() {
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
