#![allow(dead_code, unused_imports)]
/// TODO : Documentation
/// how to get valid inputs from the lib
/// how to select them out of the lib and then inform the lib
///
use std::collections::HashMap;

use dijkstra_map::DijkstraMap;
use gdnative::prelude::*;
use on_the_map::Entity;

/// everuthing doable in the world
pub mod action;
use action::*;
/// module that deals with transforming an attack into a consequence on the world
pub mod attack_solver;
/// responsible for representing the world on a 2D grid
pub mod map;
/// everything that is on the map
pub mod on_the_map;

pub mod game_manager;
pub use game_manager::*;

/// holds method for turn_logic
pub mod turn_logic;
use turn_logic::*;

/// represents teams in the game for the game manager
pub enum TeamID {
    /// maps a i32 to a team
    Team(i32),
    /// the entity has no team
    Loner,
}
/// this is the current state of the game manager
pub enum Status {
    FightNotStarted,
    EntityWaitingForInput(EntityId),
    FightEnded,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
/// allows reference to this entity for the game manager
pub struct EntityId(pub i64);

/// this is how the game_manager will communicate what choices are available for currently playing entity
/// it will be cloned and cached by the game_manager so we can use the id to declare the choice
/// spec : an id that will be used to reference the
#[derive(PartialEq, Clone)]
pub struct InputOption {
    unique_id: i32,
    action: Action,
    priority: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::on_the_map::Entity;

    fn basic_initialise_map() -> map::Map {
        map::Map::new(20, 20)
    }
    fn basic_initialise_game_manager() -> GameManager {
        let manager = GameManager::new();
        manager.initialise()
    }
    #[test]
    fn basic_test() {
        // initialize the game manager
        let mut gm = basic_initialise_game_manager();
        //  one player
        let result = gm.register_entity(Entity::example_entity(), &map::Pos2D::new(0, 0));
        result.unwrap();
        panic!()
    }
    #[test]
    fn can_retrieve_choices_from_gamemanager() {
        todo!()
    }
    #[test]
    fn _move() {
        // initialize the game manager
        // on player turn make it move somewhere
        // assert the player did move
        todo!()
    }
    #[test]
    fn _attack() {
        // initialize the game manager
        // add a second player
        // on player turn make it attack somewhere
        // assert the second player was damaged
        todo!()
    }
    /// player A attack player B, player B counterAttacks
    #[test]
    fn _counter_attack() {
        todo!()
    }
    /// player A attacks player B, a second attack will kill player B so player A attacks a second time
    #[test]
    fn double_attack_if_deadly() {
        todo!()
    }
    /// player A attacks player B, a second attack wont kill player B so player A doesnt attacks a second time
    #[test]
    fn no_double_attack_if_not_deadly() {
        todo!()
    }
    #[cfg(test)]
    mod can_attack_if {
        use super::*;

        #[test]
        fn in_range() {
            todo!()
        }
        #[test]
        fn not_if_not_in_range() {
            todo!()
        }
        #[test]
        fn loner_vs_loner() {
            todo!()
        }
        #[test]
        fn loner_vs_any_team() {
            todo!()
        }
        #[test]
        fn any_team_vs_loner() {
            todo!()
        }
        #[test]
        fn cant_if_same_team() {
            todo!()
        }
        #[test]
        fn different_teams() {
            todo!()
        }
    }
}
