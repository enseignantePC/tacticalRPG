use std::rc::Rc;

use gdnative::api::remote_transform;

use super::*;

/// This is an identifier that a [GameManager] can use to get a reference to an entity
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]

pub struct EntityId(pub i64);

/// The team of an entity,
/// TODO : entity of the same team shouldnt be able to attack each other
/// TODO : except undirectly? via a `friendly fire` option for the game manager
/// TODO : entities in the Loner team can attack anyone
#[derive(Debug)]
pub enum TeamID {
    /// a unique identifier for each team
    Team(i32),
    /// the entity has no team and can attack anyone
    Loner,
}

/// handles and connect everything
pub struct GameManager {
    /// represents the world (2D grid) and everything that is on it
    map: map::Map,
    /// how the game manager stores and references entity that are on the map
    entity_id_to_entity: HashMap<EntityId, Rc<Entity>>,
    /// Manages the intents (aka inputs) that declares how the entities want to act on the world
    intent_manager: IntentManager,
    /// resolve what effectively happens on the world and has an event system to trigger new intents to be sent according to what happened
    ///     a simple example would be: if someone attacks player A, player A always counter attacks
    ///     somehow more complex : if someone attacks player A and player A is in range of attacking, player A counter attacks
    action_manager: ActionManager,
    fight_started: bool,
    fight_ended: bool,
    entity_currently_awaiting_input: Option<EntityId>,
    ///
    input_cache: Option<InputCache>,
    world_manager: WorldManager,
    intent_watcher: Watcher,
    action_watcher: Watcher,
}
/// this is stored by the gamemanager after you chose which Entity was going to play and i gave you a choice in the form
/// of a Vec<InputOption>
pub struct InputCache {
    entity_chosen_to_play: EntityId,
    input_options: HashMap<i32, InputOption>,
}

pub struct UninitialisedGameManager {}

impl UninitialisedGameManager {
    pub fn initialise(self) -> GameManager {
        todo!()
    }
}

impl GameManager {
    pub fn new() -> UninitialisedGameManager {
        UninitialisedGameManager {}
    }
    /// adds a new entity on the map
    /// fails if the place is occupied
    /// On success, returns a entity_id that allows reference to this entity for the game manager
    pub fn register_entity(
        &mut self,
        entity: on_the_map::Entity,
        map_position: &map::Pos2D,
    ) -> Result<EntityId, ()> {
        // generate an id for the entity
        // check if the place on the map can accept the entity
        let entity_id = self.make_available_entity_id();
        if self.map.can_entity_be_accepted_at_pos(map_position) {
            self.entity_id_to_entity.insert(entity_id, Rc::new(entity));
            self.map.register_entity_at_pos(entity_id, map_position);
            return Ok(entity_id);
        }
        Err(())
    }
    /// generate valid inputs for entity
    /// what movements are okay
    /// what attacks are okay
    /// etc
    pub fn get_valid_inputs_for_entity(&mut self, entity_id: &EntityId) -> Vec<InputOption> {
        let mut result: Vec<InputOption> = Vec::new();
        let mut to_cache: HashMap<i32, InputOption> = HashMap::new();
        // used to generate id of
        let mut x = -1;
        let mut option_id_generator = move || {
            x += 1;
            x
        };

        let entity = self.entity_id_to_entity.get(entity_id).unwrap();
        let _move = self.map.get_valid_movements_for_entity(entity);
        for path in _move {
            let unique_id = option_id_generator();
            let io = InputOption {
                unique_id,

                // action: Action::Move(Move { path }),
                // // TODO : priority system for move intents
                // priority: 0i32,
                intent: Intent {
                    action: Action::Move(Move { path }),
                    priority: 0132,
                    entity: entity.clone(),
                },
            };
            result.push(io.clone());
            to_cache.insert(unique_id, io);
        }

        // TODO : attacks, objects, spell
        self.input_cache = Some(InputCache {
            entity_chosen_to_play: *entity_id,
            input_options: to_cache,
        });
        result
    }
    /// if a player p is playing its turn, give the intent for that player
    /// it consumes the cache if the input is valid
    pub fn give_inputs_according_to_cache(
        &mut self,
        id_of_valid_input_cache: i32,
    ) -> Result<(), ()> {
        if self.input_cache.is_none() {
            return Err(());
        } else {
            // get the content of the cache
            let cloned_cache = self.input_cache.take().unwrap();
            let InputCache {
                entity_chosen_to_play,
                mut input_options,
            } = cloned_cache;

            let InputOption {
                unique_id: x,
                intent,
            } = input_options.remove(&id_of_valid_input_cache).unwrap();
            self.resolve_all_intents(intent);
        };
        todo!()
    }

    /// make an entity declare an [Intent][super::turn_logic::Intent]
    fn resolve_all_intents(&mut self, intent: Intent) -> () {
        self.submit_intent_and_responses(intent);

        while !self.intent_manager.is_queue_empty() {
            let next_intent = self.intent_manager.resolve_one_intent();
            if next_intent.is_err() {
                return;
            }
            let next_intent = next_intent.unwrap();
            self.world_manager.resolve(next_intent.clone());
            let response: Vec<Intent> = self.action_watcher.watch(next_intent);
            for k in response {
                self.submit_intent_and_responses(k)
            }
        }
    }

    fn submit_intent_and_responses(&mut self, next_intent: Intent) {
        self.intent_manager.submit(next_intent.clone());
        let response: Vec<Intent> = self.intent_watcher.watch(next_intent);
        for k in response {
            self.submit_intent_and_responses(k)
        }
    }

    fn make_available_entity_id(&self) -> EntityId {
        let mut i = 0;
        while self.entity_id_to_entity.contains_key(&EntityId(i)) {
            i += 1
        }
        EntityId(i)
    }
}
