//! # The game manager module.
//!
//! The most interesting structure here is the [GameManager].
//! It is responsible of handling all the other module and making them work together
//! to offer a good interface for dealing with the intern state of the game.
use thiserror::Error;

use crate::map::AccessPositionError;

use super::*;
use std::rc::Rc;

/// This is an identifier that a [GameManager] can use to get a reference to an [Entity]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, ToVariant)]
pub struct EntityId(pub i64);

/// The team of an entity,
/// TODO : entity of the same team shouldn't be able to attack each other
/// TODO : except indirectly? via a `friendly fire` option for the game manager
/// TODO : entities in the Loner team can attack anyone
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy, ToVariant)]
pub enum TeamId {
    /// a unique identifier for each team
    Team(i64),
    /// the entity has no team and can attack anyone
    Loner,
}

impl TeamId {
    /// can entities of the supplied teams fight?
    pub fn can_fight(
        &self,
        other_team: &Self,
    ) -> bool {
        // if they are on different teams, yes
        if let (TeamId::Team(x), TeamId::Team(y)) = (self, other_team) {
            x != y
        } else {
            // if any of them are a Loner, yes
            true
        }
    }
}
#[derive(Debug)]
/// handles and connect everything
pub struct GameManager {
    /// represents the world (2D grid) and everything that is on it.
    map: map::Map,
    pub entity_id_to_entity: HashMap<EntityId, Rc<Entity>>,
    /// handles the interaction between the world and
    /// then [entities](Entity) via [Intent]s.
    intent_manager: IntentManager,
    /// handles terrain and translation to [DijkstraMap]
    terrain_manager: map::terrains::TerrainManager,
    /// watch and react to intent emitted.
    intent_watcher: Watcher,
    /// watch and react to action done.
    action_watcher: Watcher,
    /// A simple history field, storing chronologically what happens.
    world_changes: Vec<WorldChange>,
}

impl GameManager {
    /// note : terrain manager is initiated
    /// with default terrain == DEFAULT
    pub fn new(map: map::Map) -> Self {
        GameManager {
            map,
            entity_id_to_entity: HashMap::default(),
            intent_manager: IntentManager::new(),
            intent_watcher: Watcher::new(),
            action_watcher: Watcher::new(),
            world_changes: vec![],
            terrain_manager: map::terrains::TerrainManager::new("DEFAULT"),
        }
    }

    pub fn register_entity_at_pos(
        &mut self,
        entity: Box<dyn on_the_map::EntityIntern>,
        team: TeamId,
        map_position: &map::Pos2D,
    ) -> Result<Rc<Entity>, AccessPositionError> {
        // generate an id for the entity
        // check if the place on the map can accept the entity
        let entity_id = self.make_available_entity_id();
        let entity = on_the_map::Entity {
            team,
            unique_id: entity_id,
            entity_intern: entity,
        };
        let entity = Rc::new(entity);
        self.map.register_entity_at_pos(
            entity.clone(),
            map_position,
        )?;
        self.entity_id_to_entity.insert(
            entity_id,
            entity.clone(),
        );
        Ok(entity)
    }
    pub fn get_valid_intents_for_entity(
        &mut self,
        entity_id: &EntityId,
    ) -> Vec<Intent> {
        let mut result: Vec<Intent> = Vec::new();
        let entity = self.entity_id_to_entity.get(entity_id).unwrap_or_else(|| {
            panic!(
                "Tried to get intents for entity with id:{:?}\
            \nbut no such entity could be found",
                entity_id
            )
        });
        for (k, v) in entity.entity_intern.selector_map() {
            if let Some(select_result) = k.select(&mut self.map) {
                result.push(
                    entity
                        .entity_intern
                        .action_possible_to_intent(v, select_result),
                )
            }
        }
        // let _move: Vec<Intent> = self.map.get_valid_movements_for_entity(entity.clone());
        // let _attacks: Vec<Intent> = self.map.get_valid_attacks_for_entity(entity.clone());
        // let _objects: Vec<Intent> = self.map.get_valid_object_for_entity(entity.clone());
        // let _spells: Vec<Intent> = self.map.get_valid_spells_for_entity(entity.clone());
        // result.extend(_attacks);
        // result.extend(_objects);
        // result.extend(_spells);
        result
    }
    /// make an entity declare an [Intent][super::turn_logic::Intent]
    /// the intent will be `watched` (see [Watcher]) when it is emitted and when it is realized
    pub fn resolve_all_intents(
        &mut self,
        intent: Intent,
    ) -> Vec<WorldChange> {
        // stores what happens and returns it to external source
        let mut result: Vec<WorldChange> = Vec::new();

        self.submit_intent_and_responses(intent);

        while !self.intent_manager.is_queue_empty() {
            let next_intent = self.intent_manager.extract_top_intent();

            match next_intent {
                Ok(_) => {
                    let next_intent = next_intent.unwrap();
                    let world_change = self.realise_intent(&next_intent);
                    // stores the change for historic purposes
                    self.world_changes.extend(world_change.clone());
                    // watch the change
                    let response: Vec<Intent> = self.action_watcher.watch(
                        &self.entity_id_to_entity,
                        &next_intent,
                    );
                    for k in response {
                        self.submit_intent_and_responses(k)
                    }
                    result.extend(world_change);
                }
                Err(_) => break,
            }
        }
        result
    }

    /// TODO : TEST ME
    /// Returns the list of entity that should be given the choice to play now
    ///
    /// It's a list of entities of the same team that are nice enough that they
    /// would let the others play first :)
    ///
    /// ! WARNING if the entity returns can_play == true, it should return non empty input options
    /// ! when queried! Otherwise the GameManager will have trouble knowing when the turn is over.
    /// ! In the end the GameManager should become smart enough to `eliminate` a player when
    /// ! it says it can play but return no options.
    /// !
    /// ! This is subtle because the player answer if they can play according to their state,
    /// ! their actual options is not their scope.
    /// ! So how should the game manager decide when the turn is over?
    /// ! It could check before returning from get_playable_entities that the entities return
    /// ! have at least one option. Find the first team that does. If no such team exist the
    /// ! turn is terminated.
    /// !
    /// ! But what to do to avoid infinite loops in the case where all Entities could play
    /// ! but they are blocked? ...
    pub fn get_playable_entities(&mut self) -> Vec<EntityId> {
        // sort entities by initiative
        let entities: Vec<Rc<Entity>> = self.entity_id_to_entity.values().cloned().collect();
        let mut entities: Vec<Rc<Entity>> = entities
            .iter()
            .filter(|&x| x.entity_intern.can_play())
            .cloned()
            .collect();
        entities.sort_by(|a, b| {
            b.entity_intern
                .initiative()
                .partial_cmp(&a.entity_intern.initiative())
                .expect("Couldn't cmp initiative")
        });
        let mut selected: Vec<Rc<Entity>> = vec![entities.remove(0)];
        let entities: Vec<Rc<Entity>> = entities
            .into_iter()
            .take_while(|e| e.team.can_fight(&selected.first().unwrap().team))
            .collect();
        selected.extend(entities);
        selected.iter().map(|x| x.unique_id).collect()
    }

    /// this method transform an [Intent] into a [WorldChange]s
    /// and stores it in [GameManager.world_changes]
    fn realise_intent(
        &mut self,
        next_intent: &Intent,
    ) -> Vec<WorldChange> {
        let world_changes = world_manager::intent_to_world_change(
            &self.entity_id_to_entity,
            next_intent.clone(),
        );
        for world_change in &world_changes {
            world_manager::apply_change_to_world(world_change, self);
        }
        world_changes
    }
    /// submit an intent, call the intent watchers on that intent
    /// and does the same for every intention yielded by the (IntentWatcher)[Watcher], recursively
    fn submit_intent_and_responses(
        &mut self,
        next_intent: Intent,
    ) {
        self.intent_manager.submit(next_intent.clone());
        let response: Vec<Intent> = self.intent_watcher.watch(
            &self.entity_id_to_entity,
            &next_intent,
        );
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::on_the_map::Entity;

    fn basic_initialise_map() -> map::Map {
        map::Map::new(2, 2)
    }
    fn basic_initialise_game_manager() -> GameManager {
        GameManager::new(basic_initialise_map())
    }
    #[test]
    fn new() {
        // initialize the game manager
        let _gm = basic_initialise_game_manager();
    }
    #[test]
    fn can_register_one_entity() {
        // initialize the game manager
        let mut gm = basic_initialise_game_manager();
        //  one player
        let result = gm.register_entity_at_pos(
            Entity::test_entity_intern(),
            TeamId::Loner,
            &map::Pos2D::new(0, 0),
        );
        // dbg!(gm);
        result.unwrap();
    }
    // TODO fix bug where gm thinks entity is id n and entity thinks its id is not n
    #[test]
    fn cant_register_entity_at_same_pos() {
        // initialize the game manager
        let mut gm = basic_initialise_game_manager();
        //  one player
        let result = gm
            .register_entity_at_pos(
                Entity::test_entity_intern(),
                TeamId::Loner,
                &map::Pos2D::new(0, 0),
            )
            .expect("should be feasible to add an entity");
        dbg!(&gm);
        let result = gm.register_entity_at_pos(
            Entity::test_entity_intern(),
            TeamId::Loner,
            &map::Pos2D::new(0, 0),
        );
        dbg!(gm);
        if let Err(PositionOccupied) = result {
            // everything is fine
        } else {
            panic!("Adding two entities at the same position should fail")
        };
    }
    #[test]
    fn can_register_entities_at_different_pos() {
        let mut gm = basic_initialise_game_manager();
        //  one player
        gm.register_entity_at_pos(
            Entity::test_entity_intern(),
            TeamId::Loner,
            &map::Pos2D::new(0, 0),
        )
        .unwrap();
        gm.register_entity_at_pos(
            Entity::test_entity_intern(),
            TeamId::Loner,
            &map::Pos2D::new(1, 0),
        )
        .unwrap();
        gm.register_entity_at_pos(
            Entity::test_entity_intern(),
            TeamId::Loner,
            &map::Pos2D::new(0, 1),
        )
        .unwrap();
        gm.register_entity_at_pos(
            Entity::test_entity_intern(),
            TeamId::Loner,
            &map::Pos2D::new(1, 1),
        )
        .unwrap();
    }
    #[test]
    fn cant_register_entity_out_of_map() {
        let mut gm = basic_initialise_game_manager();
        //  one player
        if let Err(AccessPositionError::PositionOutOfBounds) = gm.register_entity_at_pos(
            Entity::test_entity_intern(),
            TeamId::Loner,
            &map::Pos2D::new(3, 0),
        ) {
            // all is fine
        } else {
            panic!("We should get an out of bounds!")
        }
    }
    //test that when entity is moved, all fields are updated in consequence
}
//     #[test]
//     fn can_retrieve_choices_from_game_manager() {
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
//     /// player A attacks player B, a second attack wont kill player B so player A doesn't attacks a second time
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
