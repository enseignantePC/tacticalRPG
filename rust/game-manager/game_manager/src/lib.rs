#![allow(dead_code, unused_imports)]
use dijkstra_map::DijkstraMap;
use gdnative::prelude::*;

/// responsible for representing the world on a 2D grid
pub mod map;

/// holds method for turn_logic
pub mod turn_logic;

/// everything that is on the map
pub mod on_the_map;

/*
/// everything that is on the map has this trait
///
/// means you can generate PlayOptions by interacting with this
pub trait Interactable {}*/

/// represents teams in the game for the game manager
pub enum TeamID {
    /// maps a i32 to a team
    Team(i32),
    /// the entity has no team
    Loner,
}
/// represents an action that happenned in the world
pub enum Consequences {}
///
pub struct GameManager {}
pub struct UninitialisedGameManager {}

impl UninitialisedGameManager {
    fn initialise() -> GameManager {
        todo!()
    }
}

impl GameManager {
    fn new() -> UninitialisedGameManager {
        todo!()
    }
    /// adds a new entity on the map
    /// fails if the place is occupied
    fn register_entity(entity: on_the_map::Entity, map_position: map::Pos2D) -> Result<(), ()> {
        todo!()
    }
    ///
    fn play_turn() {
        todo!()
    }
    /// if a player p is playing its turn, give the intent for that player
    fn give_intent() -> Vec<Consequences> {
        todo!()
    }
    /// ask who is playing and what are his options, is the game finished? etc
    fn ask_status() {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn basic_initialise_map() -> map::Map {
        todo!()
    }
    fn basic_initialise_game_manager() {
        let manager = GameManager::new();
        todo!()
    }
    #[test]
    fn basic_test() {
        panic!()
        // initialize the game manager
        //  give it a map
        //  one player
    }
    #[test]
    fn _move() {
        // initialize the game manager
        // on player turn make it move somewhere
        todo!()
    }
    #[test]
    fn _attack() {
        // initialize the game manager
        // add a second player
        // on player turn make it attack somewhere
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
}
