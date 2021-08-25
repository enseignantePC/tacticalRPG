use gdnative::api::remote_transform;

use super::*;
/// handles and connect everything
pub struct GameManager {
    /// represents the world (2D grid) and everything that is on it
    map: map::Map,
    /// how the game manager stores and references entity that are on the map
    entity_id_to_entity: HashMap<EntityId, Entity>,
    /// Manages the intents (aka inputs) that declares how the entities want to act on the world
    intent_manager: IntentManager,
    /// resolve what effectively happens on the world and has an event system to trigger new intents to be sent according to what happened
    ///     a simple example would be: if someone attacks player A, player A always counter attacks
    ///     somehow more complex : if someone attacks player A and player A is in range of attacking, player A counter attacks
    action_manager: ActionManager,
    fight_started: bool,
    fight_ended: bool,
    entity_currently_awaiting_input: Option<EntityId>,
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
        map_position: map::Pos2D,
    ) -> Result<EntityId, ()> {
        // generate an id for the entity
        // check if the place on the map can accept the entity
        let entity_id = self.make_available_entity_id();
        if self.map.can_entity_be_accepted_at_pos(map_position.clone()) {
            self.entity_id_to_entity.insert(entity_id, entity);
            self.map.register_entity_at_pos(entity_id, map_position);
            return Ok(entity_id);
        }
        Err(())
    }
    /// generate valid inputs for entity
    /// what movements are okay
    /// what attacks are okay
    /// etc
    pub fn get_valid_inputs(&mut self, entity_id: EntityId) {
        self.map.get_valid_movement_for_entity_at_pos(
            self.entity_id_to_entity.get(&entity_id).unwrap(),
            todo!(),
        );

        todo!()
    }
    /// if a player p is playing its turn, give the intent for that player
    pub fn give_inputs(&mut self, intent: Intent) {
        // submit a new input
        self.intent_manager.submit(intent);
    }

    /// resolve intents one by one, alerting the event system
    /// until it needs an input to continue
    /// when over
    ///     map has been modified (spell or movement or object dissapearance, death)
    ///     or entities states have been altered (attacks, death)
    ///
    ///     information needed :
    ///         what_happenned
    ///         status
    ///         if continue currently playing entity needing input
    /// TODO
    pub fn poll(&mut self) {
        let intent = self.intent_manager.resolve_one_intent();
        match intent {
            Ok(intent) => self.action_manager.resolve(intent.action),
            Err(_) => todo!(),
        }
    }
    /// ask who is playing and what are his options, is the game finished? etc
    pub fn ask_status(&self) -> Status {
        if !self.fight_started {
            Status::FightNotStarted
        } else if self.fight_ended {
            Status::FightEnded
        } else {
            Status::EntityWaitingForInput(self.entity_currently_awaiting_input.expect("If the fight is still going, there always should be an entity waiting for input at this point"))
        }
    }
    /// declares the setup over and the fight started! this can fail if the setup was not sufficient! (nobody on the map)
    /// ? TODO : move this to UnititiliasedGameManager
    pub fn start_fight() {
        todo!()
    }

    ///
    fn make_available_entity_id(&self) -> EntityId {
        let mut i = 0;
        while self.entity_id_to_entity.contains_key(&EntityId(i)) {
            i += 1
        }
        EntityId(i)
    }
}
