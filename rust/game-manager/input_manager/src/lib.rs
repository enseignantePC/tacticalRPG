//! This module is responsible of offering a clean interface to an external source.
//! The [InputManager] will be the access point from which you can:
//! - query what is possible to do for currently playing [Entity] via [InputOption]
//! - answer what the entity will do via ~ dont know yet, an _answer method or smt

use std::collections::HashMap;

use ::game_manager::thiserror::Error;
use ::game_manager::Intent;
use ::game_manager::WorldChange;
use ::game_manager::*;

/// this is how a [GameManager] will communicate what choices are available for currently playing entity
/// it will be cloned and cached by the game_manager so we can use the id to declare the choice
/// spec : an id that will be used to reference the
#[derive(Clone)]
pub struct InputOption {
    /// this id should be unique and will be communicated to the game manager
    /// to ensure this specific [InputOption] will be selected
    pub unique_id: i32,
    /// the [Intent] corresponding
    pub intent: Intent,
}

/// The InputManager is responsible for communicating
/// with an external sources that will provide inputs.
///
/// It keeps track of the state of the game to always permit to be asked what
/// - inputs must be submitted
/// - what is the context/ what are the options etc
///
pub struct InputManager {
    /// stores what input are available after a query via [GameManager::get_valid_inputs_for_entity]
    game_manager: GameManager,
    input_cache: Option<InputCache>,
    currently_playing: EntityId,
}

/// this is stored by the [InputManager] after you chose which Entity was going to play and i gave you a choice in the form
/// of a Vec<InputOption>
pub struct InputCache {
    entity_chosen_to_play: EntityId,
    input_options: HashMap<i32, InputOption>,
}
#[derive(Error, Debug)]
#[error("Trying to access an InputCache that is none")]
pub struct InputCacheIsNone;

impl InputManager {
    pub fn get_valid_intents_for_entity(
        &mut self,
        entity_valid_intents: &[Intent],
    ) -> HashMap<i32, InputOption> {
        let mut to_cache: HashMap<i32, InputOption> = HashMap::new();
        // used to generate id of
        let mut x = -1;
        let mut option_id_generator = move || {
            x += 1;
            x
        };
        for (_, v) in entity_valid_intents.iter().enumerate() {
            let x = option_id_generator();
            let io = InputOption {
                unique_id: x,
                intent: v.clone(),
            };
            to_cache.insert(x, io);
        }
        self.input_cache = Some(InputCache {
            entity_chosen_to_play: self.currently_playing,
            input_options: to_cache.clone(),
        });
        to_cache
    }

    /// if a player p is playing its turn, give the intent for that player
    /// it consumes the cache if the input is valid
    ///
    /// returns a vector of the [WorldChange]s that happened in the world
    /// fails if the input is invalid (aka, its unique id doesn't exist in the [InputCache])
    pub fn give_inputs_according_to_cache(
        &mut self,
        id_of_valid_input_cache: i32,
    ) -> Result<Vec<WorldChange>, InputCacheIsNone> {
        if self.input_cache.is_none() {
            Err(InputCacheIsNone {})
        } else {
            // get the content of the cache
            let cloned_cache = self.input_cache.take().unwrap();
            let InputCache {
                entity_chosen_to_play: _,
                mut input_options,
            } = cloned_cache;

            let InputOption {
                unique_id: _,
                intent,
            } = input_options.remove(&id_of_valid_input_cache).unwrap();

            Ok(self.game_manager.resolve_all_intents(intent))
        }
    }
}
