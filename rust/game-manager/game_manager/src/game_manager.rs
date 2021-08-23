use super::*;
pub struct GameManager {
    map: map::Map,
    entity_id_to_entity: HashMap<EntityId, Entity>,
}
pub struct UninitialisedGameManager {}

impl UninitialisedGameManager {
    pub fn initialise(self) -> GameManager {
        todo!()
    }
}

impl GameManager {
    pub fn new() -> UninitialisedGameManager {
        todo!()
    }
    /// adds a new entity on the map
    /// fails if the place is occupied
    /// On success, returns a entity_id that allows reference to this entity for the game manager
    pub fn register_entity(
        &mut self,
        entity: on_the_map::Entity,
        map_position: map::Pos2D,
    ) -> Result<(), ()> {
        // generate an id for the entity
        // check if the place on the map can accept the entity
        let entity_id = self.make_available_entity_id();
        if self.map.can_entity_be_accepted_at_pos(map_position.clone()) {
            self.entity_id_to_entity.insert(entity_id, entity);
            self.map.register_entity_at_pos(entity_id, map_position);
        }
        todo!()
    }
    /// if a player p is playing its turn, give the intent for that player
    pub fn give_inputs(&self) -> Vec<Consequences> {
        // submit a new input
        // resolve all inputs, storing what happens
        // when over, caches what_happens
        // get currently playing entity
        // submit their intent or fails with InvalidIntents
        // return the consequences
        todo!()
    }
    /// ask who is playing and what are his options, is the game finished? etc
    pub fn ask_status(&self) -> Status {
        todo!()
    }
    ///
    fn make_available_entity_id(&self) -> EntityId {
        todo!()
    }
}
