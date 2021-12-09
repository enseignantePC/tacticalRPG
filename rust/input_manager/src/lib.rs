//! # The Input Manager
//!
//! It provides a clean interface to an external source.
//! The [InputManager] will be the access point from which you can:
//! - query what is possible to do for currently playing [Entity] via [InputOption]
//!
//!
//! - [] Query what entities can play
//! - [] Query what intents can be emitted for one of this entity
//! - [] Choose one of these actions
//!
//! right now, if you provide an entity, you get a map of id -> Valid Intent
//! TODO
//! You will get a map of id -> (ActionKind, parameters)
//! for instance 0 -> Movement, list of positions
//! TODO

use ::game_manager::thiserror::Error;
use ::game_manager::{EntityId, GameManager, Intent, WorldChange};
use std::collections::HashMap;

mod godot_interface;

/// The [InputManager] communicates with an external source that  provide inputs
/// and the [GameManager] that updates in consequence of these inputs (and also
/// provides valid inputs).
///
/// you ask what entities can play with query_playable_entities
/// you then query what they can do via query_entity_options
/// you then choose what an entity will do choose_action_for_entity
pub struct InputManager {
    /// stores what input are available after
    /// a query via [GameManager::get_valid_inputs_for_entity]
    game_manager: GameManager,
    /// A cache with the play_options for the entity
    input_cache: Option<(InputCache, EntityId)>,
    playable_entities: Option<Vec<EntityId>>,
}

/// this is stored by the [InputManager] after you chose which
/// Entity was going to play and i gave you a choice in the form
/// of a Vec<InputOption>
pub type InputCache = HashMap<i32, InputOption>;

/// this is how a [GameManager] will communicate what choices are available
/// the chosen entity.
///
/// A copy is kept by the input manager so you provide a key for selecting your
/// option (and not an intent, so you can't make up an illegal one you filthy
/// bastard yes you 💕).
#[derive(Clone)]
pub struct InputOption {
    /// A unique id that can be communicated back to the
    /// [InputManager] to ensure the intent field is selected.
    pub unique_id: i32,
    /// a valid (means it was provided by the game manager) [Intent].
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
