use std::{cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc};

use crate::{
    common_types::{
        Action, Entity, EntityId, Intent, Position, Selector, SelectorResult, TeamId, WorldChange,
    },
    map::Map,
};

#[derive(Debug)]
pub struct EntityIntern<T: Entity> {
    pub entity: T,
    pub id: EntityId,
    pub team_id: TeamId,
}

pub struct GameManagerInitialiser {}

impl GameManagerInitialiser {
    pub fn initialise<T: Entity>(
        // pub fn initialise<T: AttackResolver>(
        map: Map,
        // attack_resolver: T,
    ) -> GameManager<T> {
        GameManager {
            map: RefCell::new(map),
            entities: HashMap::new(),
            gen_id: 0,
        }
    }
}
#[derive(Debug)]
pub struct GameManager<T: Entity> {
    map: RefCell<Map>,
    entities: HashMap<EntityId, Rc<EntityIntern<T>>>,
    gen_id: i32,
}

impl<EntityImpl: Entity> GameManager<EntityImpl> {
    /// returns a unique entity id that must be use to reference the entity registered

    pub fn register(
        &mut self,
        entity: EntityImpl,
        team_id: TeamId,
    ) -> EntityId {
        let id = self.get_next_id();
        let e = EntityIntern {
            entity,
            id: id.clone(),
            team_id,
        };
        self.entities.insert(
            id.clone(),
            Rc::new(e),
        );
        id
    }
    /// fails if position occupied or out of bounds
    pub fn try_place(
        &self,
        entity: EntityId,
        pos: Position,
    ) -> Result<(), ()> {
        if self.map.borrow().is_out_of_bounds(pos) {
            return Err(());
        }
        if self.map.borrow().is_occupied(pos) {
            return Err(());
        }
        self.map.borrow_mut().place(entity, pos);
        Ok(())
    }

    pub fn unplace(
        &self,
        entity: EntityId,
    ) {
        self.map.borrow_mut().unplace(entity);
    }

    pub fn submit_intent(
        &mut self,
        intent: Intent,
    ) -> Vec<WorldChange<EntityImpl>> {
        self.insert_intent_in_queue(intent.clone());
        for i in self.watch_intent_submitted(intent).iter().cloned() {
            self.submit_intent(i);
        }
        let res = self.resolve_intents_submitted();
        self.apply_changes_to_world(res.clone());
        res
    }

    pub fn resolve_intents_submitted(&mut self) -> Vec<WorldChange<EntityImpl>> {
        let mut res: Vec<WorldChange<EntityImpl>> = vec![];
        while self.intent_queue_not_empty() {
            let i = self.pop_higher_priority_intent();
            res.append(&mut Self::apply_intent(i.clone()));
            self.watch_intent_resolved(i);
        }
        res
    }

    pub fn get_playable_entities(&self) -> Vec<EntityId> {
        // sort by initiative
        // sort by same team
        // let entity_queue = self.get_playable_entity_sorted();
        //
        // let first_id = dbg!(&entity_queue).first();
        // check up
        let mut entity_queue: Vec<EntityId> = self
            .entities
            .keys()
            .copied()
            .filter(|x| self.get_entity(*x).entity.can_still_play())
            .collect();
        entity_queue.sort_by(|x, y| {
            let (x, y) = (
                self.get_entity(*x),
                self.get_entity(*y),
            );
            x.entity
                .get_initiative()
                .partial_cmp(&y.entity.get_initiative())
                .unwrap()
        });
        let first_id = entity_queue.first();
        if let Some(first_id) = first_id {
            let team = self.get_entity(*first_id).team_id;
            let mut res: Vec<EntityId> = vec![*first_id];
            for id in entity_queue.iter() {
                if self.get_entity(*id).team_id.is_ally(&team) {
                    res.push(*id);
                } else {
                    break;
                }
            }
            res
        } else {
            vec![]
        }
    }

    fn get_playable_entity_sorted(&self) -> Vec<EntityId> {
        let mut entity_queue: Vec<EntityId> = self.entities.keys().copied().collect();
        entity_queue.sort_by(|x, y| {
            let (x, y) = (
                self.get_entity(*x),
                self.get_entity(*y),
            );
            y.entity
                .get_initiative()
                .partial_cmp(&x.entity.get_initiative())
                .unwrap()
        });
        entity_queue
            .into_iter()
            .filter(|&x| self.get_entity(x).entity.can_still_play())
            .collect()
    }

    pub fn get_play_options_for(
        &self,
        e: EntityId,
    ) -> Vec<(Selector, Action)> {
        self.get_entity(e).entity.get_play_options()
    }

    pub fn map_select(
        &self,
        id: EntityId,
        selector: Selector,
    ) -> SelectorResult {
        match selector.mode {
            crate::common_types::SelectorMode::Djikstra { move_force } => {
                // bake map for entity
                self.map.borrow_mut().recalculate_for_entity(
                    &self.get_entity(id),
                    move_force,
                );
                // perform the search (disabling the points that need to be disabled and reenabling them afterwards)
                todo!()

                // self.map.borrow_mut().select(selector)
            }
        }

        impl<EntityImpl: Entity> GameManager<EntityImpl> {
            fn get_entity(
                &self,
                id: EntityId,
            ) -> Rc<EntityIntern<EntityImpl>> {
                self.entities.get(&id).unwrap().clone()
            }

            fn move_entity(
                &mut self,
                entity: EntityId,
                new_pos: Position,
            ) {
                todo!()
            }

            fn get_next_id(&mut self) -> EntityId {
                self.gen_id += 1;
                EntityId(self.gen_id)
            }

            fn watch_intent_submitted(
                &self,
                intent: Intent,
            ) -> Vec<Intent> {
                todo!()
            }

            fn watch_intent_resolved(
                &self,
                intent: Intent,
            ) -> Vec<Intent> {
                todo!()
            }

            fn insert_intent_in_queue(
                &mut self,
                intent: Intent,
            ) {
                todo!()
            }

            fn intent_queue_not_empty(&self) -> bool {
                todo!()
            }

            fn pop_higher_priority_intent(&mut self) -> Intent {
                todo!()
            }

            fn apply_intent(intent: Intent) -> Vec<WorldChange<EntityImpl>> {
                todo!()
            }

            fn apply_changes_to_world(
                &mut self,
                changes: Vec<WorldChange<EntityImpl>>,
            ) {
                for change in changes.iter() {
                    match change {
                        WorldChange::EntityMoved { id, from, to } => self.move_entity(*id, *to),
                        WorldChange::EntitySentMessage { from, to, msg } => todo!(),
                        WorldChange::EntityStateChanged { id, change } => todo!(),
                        WorldChange::EntityPlaced(_, _) => todo!(),
                        WorldChange::EntityUnplaced(_) => todo!(),
                        // WorldChange::EntityStateChanged(_) => {}
                        // WorldChange::EntityMoved(entity, new_pos) => self.move_entity(*entity, *new_pos),
                        // WorldChange::EntityUnplaced(entity) => self.unplace(*entity),
                        // WorldChange::EntityPlaced(entity, pos) => self.try_place(*entity, *pos).unwrap(),
                    }
                }
            }
        }
    }
}
