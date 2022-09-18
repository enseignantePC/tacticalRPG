use std::{cell::RefCell, collections::HashMap, marker::PhantomData};

use crate::{
    common_types::{Entity, EntityId, Intent, Position, Selector, SelectorResult, WorldChange},
    map::Map,
};

pub struct EntityIntern<T: Entity> {
    pub entity: T,
    pub id: EntityId,
}

pub struct GameManagerInitialiser {}

impl GameManagerInitialiser {
    pub fn initialise<T: Entity>(
        // pub fn initialise<T: AttackResolver>(
        map: Map,
        // attack_resolver: T,
    ) -> GameManager<T> {
        todo!()
    }
}
pub struct GameManager<T: Entity> {
    map: RefCell<Map>,
    entities: HashMap<EntityId, EntityIntern<T>>,
    gen_id: i32,
}

impl<EntityImpl: Entity> GameManager<EntityImpl> {
    /// returns a unique entity id that must be use to reference the entity registered

    pub fn register(
        &mut self,
        entity: EntityImpl,
    ) -> EntityId {
        let id = self.get_next_id();
        let e = EntityIntern {
            entity,
            id: id.clone(),
        };
        self.entities.insert(id.clone(), e);
        id
    }

    pub fn try_place(
        &self,
        entity: EntityId,
        pos: Position,
    ) -> Result<(), ()> {
        todo!()
    }

    pub fn unplace(
        &self,
        entity: EntityId,
    ) {
        todo!()
    }

    pub fn submit_intent(
        &mut self,
        intent: Intent,
    ) -> Vec<WorldChange<EntityImpl::EntityChange>> {
        self.insert_intent_in_queue(intent.clone());
        for i in self.watch_intent_submitted(intent).iter().cloned() {
            self.submit_intent(i);
        }
        let res = self.resolve_intents_submitted();
        self.apply_changes_to_world(res.clone());
        res
    }

    pub fn resolve_intents_submitted(&mut self) -> Vec<WorldChange<EntityImpl::EntityChange>> {
        let mut res: Vec<WorldChange<EntityImpl::EntityChange>> = vec![];
        while self.intent_queue_not_empty() {
            let i = self.pop_higher_priority_intent();
            res.append(&mut Self::apply_intent(i.clone()));
            self.watch_intent_resolved(i);
        }
        res
    }

    pub fn map_select(
        &self,
        selector: Selector,
    ) -> SelectorResult {
        self.map.borrow_mut().select(selector)
    }
}

impl<EntityImpl: Entity> GameManager<EntityImpl> {
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

    fn apply_intent(intent: Intent) -> Vec<WorldChange<EntityImpl::EntityChange>> {
        todo!()
    }

    fn apply_changes_to_world(
        &mut self,
        changes: Vec<WorldChange<EntityImpl::EntityChange>>,
    ) {
        todo!()
    }
}
