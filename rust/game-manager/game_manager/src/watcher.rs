use std::rc::Rc;

use crate::on_the_map::EntityIntern;

use super::*;

/// A Watcher structure
/// - Accept subscriptions of EventWatchers
/// - It can be given an intent
/// (typically because it will be or has been sent
/// to the world to be resolved into consequences).
/// It returns itself a Vec of Intentions
/// that are the consequences wanted by the EventWatchers.
///
/// Issuing Consequences has Side effect(?) as an entity might not
/// be capable of watching the same events multiple times?
///
/// TODO : add a way for event_watchers to signal they are not useful anymore (and can be dropped).
pub struct Watcher {
    event_watchers: Vec<(
        EntityId,
        Box<dyn EventWatchers>,
    )>,
}

impl Watcher {
    /// when an intent is watched, every watcher that subscribed will look at it and decide if its
    /// going to react or not.
    ///
    /// The returned value is a vector of [Intents][Intent] that the watchers have generated as a response.
    pub fn watch(
        &mut self,
        id_to_entity: &HashMap<EntityId, Rc<Entity>>,
        intent: &Intent,
    ) -> Vec<Intent> {
        let mut result = Vec::new();
        for (entity_id, watcher) in &self.event_watchers {
            let watcher_entity = match id_to_entity.get(entity_id) {
                Some(x) => x,
                None => continue,
            };
            if watcher.watch(
                watcher_entity,
                intent,
            ) {
                let react_intent = watcher.react(
                    watcher_entity,
                    intent,
                );
                result.push(react_intent)
            };
        }
        result
    }

    /// To add a new event watcher to the watcher
    pub fn subscribe(
        &mut self,
        entity_id: EntityId,
        event_watcher: Box<dyn EventWatchers>,
    ) {
        self.event_watchers.push((
            entity_id,
            event_watcher,
        ))
    }
}

/// This trait must be implemented to be able to subscribe in a Watcher struct
pub trait EventWatchers {
    /// When watching an Intent, an EventWatcher must be capable of
    /// deciding if an intent will trigger a reaction.
    fn watch(
        &self,
        own_entity: &Entity,
        intent_analysed: &Intent,
    ) -> bool;
    /// When an intent triggers a reaction, the watcher must updates itself and its entity
    /// and issue an intent as a reaction
    fn react(
        &self,
        own_entity: &Entity,
        intent_analysed: &Intent,
    ) -> Intent;
}

// TODO test all conditions and that they work, but before that, have a way to
// TODO create them efficiently.
// TODO a module with condition, some struct that impl EventWatchers and auto disable after n turn or n.watch
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        todo!()
    }
}
