//! The most interesting structure here is the [GameManager]
//!
use super::*;
use std::rc::Rc;

/// This is an identifier that a [GameManager] can use to get a reference to an entity
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, ToVariant)]
pub struct EntityId(pub i64);

/// The team of an entity,
/// TODO : entity of the same team shouldnt be able to attack each other
/// TODO : except undirectly? via a `friendly fire` option for the game manager
/// TODO : entities in the Loner team can attack anyone
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy, ToVariant)]
pub enum TeamId {
    /// a unique identifier for each team
    Team(i32),
    /// the entity has no team and can attack anyone
    Loner,
}

impl TeamId {
    /// can entities of the suplied teams fight?
    pub fn can_fight(&self, other_team: &Self) -> bool {
        // if they are on different teams, yes
        if let (TeamId::Team(x), TeamId::Team(y)) = (self, other_team) {
            x != y
        } else {
            // if any of them are a Loner, yes
            true
        }
    }
}

/// handles and connect everything
pub struct GameManager {
    /// represents the world (2D grid) and everything that is on it
    map: map::Map,
    /// how the game manager stores and references entity that are on the map
    entity_id_to_entity: HashMap<EntityId, Rc<Entity>>,
    /// Manages the intents (aka inputs) that declares how the entities want to act on the world
    intent_manager: IntentManager,
    fight_started: bool,
    fight_ended: bool,
    entity_currently_awaiting_input: Option<EntityId>,
    /// stores what input are available after a query via [GameManager::get_valid_inputs_for_entity]
    input_cache: Option<InputCache>,
    /// watch and react to intent emitted
    intent_watcher: Watcher,
    /// watch and react to action done
    action_watcher: Watcher,
    /// A simple history field, storing chronologically what happens
    world_changes: Vec<WorldChange>,
}
/// this is stored by the gamemanager after you chose which Entity was going to play and i gave you a choice in the form
/// of a Vec<InputOption>
pub struct InputCache {
    entity_chosen_to_play: EntityId,
    input_options: HashMap<i32, InputOption>,
}

impl GameManager {
    pub fn register_entity(
        &mut self,
        entity: on_the_map::Entity,
        map_position: &map::Pos2D,
    ) -> Result<EntityId, ()> {
        // generate an id for the entity
        // check if the place on the map can accept the entity
        let entity_id = self.make_available_entity_id();
        let entity = Rc::new(entity);
        if self.map.can_entity_be_accepted_at_pos(map_position) {
            self.entity_id_to_entity.insert(entity_id, entity.clone());
            self.map
                .register_entity_at_pos(entity.clone(), map_position);
            return Ok(entity_id);
        }
        Err(())
    }
    /// generate valid inputs for entity
    /// - what movements are okay
    /// - what attacks are okay
    /// - etc
    ///
    /// stores and return a hashmap of the form :
    ///         unique id -> valid_intent
    ///
    /// input can then be submitted in the form of that unique id
    /// via the method [GameManager.give_inputs_according_to_cache]
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
                intent: Intent {
                    action: Action::Move(Move { path }),
                    // TODO : priority system
                    priority: 0i32,
                    entity: entity.clone(),
                },
            };
            result.push(io.clone());
            to_cache.insert(unique_id, io);
        }
        // TODO : attacks
        // TODO : objects
        // TODO : spell
        self.input_cache = Some(InputCache {
            entity_chosen_to_play: *entity_id,
            input_options: to_cache,
        });
        result
    }
    /// if a player p is playing its turn, give the intent for that player
    /// it consumes the cache if the input is valid
    ///
    /// returns a vector of the [WorldChange]s that happened in the world
    /// fails if the input is invalid (aka, its unique id doesnt exist in the [InputCache])
    pub fn give_inputs_according_to_cache(
        &mut self,
        id_of_valid_input_cache: i32,
    ) -> Result<Vec<WorldChange>, ()> {
        if self.input_cache.is_none() {
            Err(())
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
            Ok(self.resolve_all_intents(intent))
        }
    }

    /// make an entity declare an [Intent][super::turn_logic::Intent]
    /// the intent will be `watched` (see [Watcher]) when it is emitted and when it is realised
    fn resolve_all_intents(&mut self, intent: Intent) -> Vec<WorldChange> {
        // stores what happens and returns it to external source
        let result: Vec<WorldChange> = Vec::new();

        self.submit_intent_and_responses(intent);

        while !self.intent_manager.is_queue_empty() {
            let next_intent = self.intent_manager.extract_top_intent();
            if next_intent.is_err() {
                return result;
            } else {
                let next_intent = next_intent.unwrap();
                let world_change = self.realise_intent(&next_intent);
                // stores the change for historic purposes
                self.world_changes.push(world_change.clone());
                // watch the change
                let response: Vec<Intent> = self.action_watcher.watch(&next_intent);
                for k in response {
                    self.submit_intent_and_responses(k)
                }
            }
        }
        result
    }
    /// this method transform an intent into a worldchange and stores it in [GameManager.world_changes]
    /// this is where something that was wanted by an entity finally becomes reality
    fn realise_intent(&mut self, next_intent: &Intent) -> Vec<WorldChange> {
        let world_changes = world_manager::intent_to_world_change(next_intent.clone());
        for world_change in &world_changes {
            world_manager::apply_change_to_world(world_change, &mut self.map);
        }
        world_changes
    }
    /// submit an intent, call the intent watchers on that intent
    /// and does the same for every intention yielded by the intent[Watcher], recursively
    fn submit_intent_and_responses(&mut self, next_intent: Intent) {
        self.intent_manager.submit(next_intent.clone());
        let response: Vec<Intent> = self.intent_watcher.watch(&next_intent);
        for k in response {
            self.submit_intent_and_responses(k)
        }
    }
    /// generates a unique, unused EntityId
    fn make_available_entity_id(&self) -> EntityId {
        let mut i = 0;
        while self.entity_id_to_entity.contains_key(&EntityId(i)) {
            i += 1
        }
        EntityId(i)
    }
}
