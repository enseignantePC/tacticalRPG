//! # The Input Manager
//!
//! It provides a clean interface to an external source.
//! The [InputManager] will be the access point from which you can:
//! - query what is possible to do for currently playing [game_manager::Entity] via [InputOption]
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
    game_manager: GameManager,
    /// stores what input are available for an [game_manager::Entity] after
    /// a query via [InputManager::query_entity_options]
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

#[derive(Error, Debug)]
pub enum ChooseActionError {
    #[error(
        "Trying to make a choice for entity but input cache is None.\
    \nHave you called query_entity_options?"
    )]
    InputCacheNull,
    #[error(
        "Trying to make a choice for entity but the \
        key provided didn't correspond to any result."
    )]
    IllegalChoice,
}

#[derive(Error, Debug)]
pub enum QueryEntityOptionsError {
    #[error(
        "Trying to query entity options but playable\
     entities cache is None, have you called \
     query_playable_entities?"
    )]
    PlayableEntitiesCacheNull,
    #[error(
        "Trying to query entity options \
    for an entity not in the cache.\
    Maybe you have some confused code somewhere? \
    Be wary when you cast an unknown u32 to an EntityId,\
    they are not to be trusted!"
    )]
    EntityRequestedNotFound,
}

impl InputManager {
    pub fn new(game_manager: GameManager) -> Self {
        InputManager {
            game_manager,
            input_cache: None,
            playable_entities: None,
        }
    }

    /// returns a list of id's corresponding to
    /// valid entities that can play
    /// and updates the cache.
    pub fn query_playable_entities(&mut self) -> Vec<EntityId> {
        let get_playable_entities = self.game_manager.get_playable_entities();
        self.playable_entities = Some(get_playable_entities.clone());
        get_playable_entities
    }

    pub fn query_entity_options(
        &mut self,
        entity_id: EntityId,
    ) -> Result<InputCache, QueryEntityOptionsError> {
        if self.playable_entities.is_some()
            && self.playable_entities.take().unwrap().contains(&entity_id)
        {
            let valid_intents_for_entity =
                self.game_manager.get_valid_intents_for_entity(&entity_id);

            let cache = Self::_intents_to_cache(
                entity_id,
                valid_intents_for_entity.as_slice(),
            );
            self.input_cache = Some((
                cache.clone(),
                entity_id,
            ));
            Ok(cache)
        } else if self.playable_entities.is_none() {
            Err(QueryEntityOptionsError::PlayableEntitiesCacheNull)
        } else {
            Err(QueryEntityOptionsError::EntityRequestedNotFound)
        }
    }

    /// Make an entity try to do an Action.
    ///
    /// Returns a vector of the [WorldChange]s that happened in the world
    /// as a consequence of the input.
    ///
    /// Or fails if the input is invalid (aka, its unique id doesn't exist in
    /// the [InputCache]).
    ///
    /// You must typically call query_playable_entities and query_entity_options to get
    /// the choice id.
    ///
    /// !WARNING : it is currently the responsibility of the input manager
    /// !to forbid illegal inputs, current system can be abused! It should
    /// !be implemented as States built into types instead.
    pub fn choose_action_for_entity(
        &mut self,
        choice_id: i32,
    ) -> Result<Vec<WorldChange>, ChooseActionError> {
        if self.input_cache.is_none() {
            Err(ChooseActionError::InputCacheNull)
        } else {
            // get the content of the cache
            let (cloned_cache, _) = self.input_cache.take().unwrap();

            if let Some(input_option) = cloned_cache.get(&choice_id) {
                Ok(self
                    .game_manager
                    .resolve_all_intents(input_option.intent.clone()))
            } else {
                Err(ChooseActionError::IllegalChoice)
            }
        }
    }

    fn _intents_to_cache(
        _: EntityId,
        valid_intents_for_entity: &[Intent],
    ) -> InputCache {
        let mut map: HashMap<i32, InputOption> = HashMap::new();
        // used to generate id of
        let mut x = -1;
        let mut option_id_generator = move || {
            x += 1;
            x
        };
        for (_, v) in valid_intents_for_entity.iter().enumerate() {
            let x = option_id_generator();
            let io = InputOption {
                unique_id: x,
                intent: v.clone(),
            };
            map.insert(x, io);
        }
        map
    }
}
