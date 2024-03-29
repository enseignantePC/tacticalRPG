use std::rc::Rc;

use thiserror::Error;

use crate::map::select::SelectorResult;

use super::*;

#[derive(Debug, Error)]
#[error("Queue empty was reached but this shouldn't be possible")]
pub struct QueueEmptyReached;
/// represents the intention of an [Entity] to do any kind of [Action] in the world
#[derive(Debug, Clone)]
pub struct Intent {
    /// the action the entity wants to accomplish
    pub action: Action,
    /// an indicator that changes how fast the intent will be treated by the [IntentManager],
    /// higher priority means the intent will be treated faster.
    pub priority: i64,
    /// a reference to the entity that wants to do the action
    pub entity: Rc<Entity>,
    pub targets: Option<SelectorResult>,
}

/// representation of what an entity wants to do

impl Intent {
    /// split the intent in two :
    /// - the smallest part possible of the intent that should be use to immediately change the world
    /// - the remainder of the intent, that should be put back on the intent queue, and treated before
    /// newer intent are submitted to the [IntentManager]
    ///
    /// Currently this correspond to
    /// - only walking on step when moving
    /// TESTME : if the remainder path becomes empty, the remainder intent returned is None
    /// TESTME : the remainder intent is always smaller that the original intent
    pub fn extract_minimal_intent(
        self
    ) -> (
        Intent,
        Option<Intent>,
    ) {
        match self.action {
            Action::Attack(_attack) => (self, None),

            Action::Move(mut _move) => {
                let next_path = vec![_move.path.remove(0)];
                let remainder_should_be_none = _move.path.is_empty();
                let (minimal_intent, remainder_intent) = (
                    Intent {
                        action: Action::Move(Move { path: next_path }),
                        priority: self.priority,
                        entity: self.entity.clone(),
                        targets: self.targets.clone(),
                    },
                    Intent {
                        action: Action::Move(Move { path: _move.path }),
                        priority: self.priority,
                        entity: self.entity.clone(),
                        targets: self.targets.clone(),
                    },
                );
                let remainder_intent = if remainder_should_be_none {
                    None
                } else {
                    Some(remainder_intent)
                };
                (
                    minimal_intent,
                    remainder_intent,
                )
            }

            Action::Object(_) => (self, None),
            Action::Spell(_) => (self, None),
        }
    }
    //
    #[cfg(test)]
    pub fn test_intent(
        action: Option<Action>,
        priority: Option<i64>,
        entity: Option<i64>,
        entity_id: Option<i64>,
    ) -> Intent {
        Intent {
            action: action.unwrap_or_else(|| Action::test_action(ActionKind::Move)),
            priority: priority.unwrap_or(0),
            entity: Rc::new(Entity::test_entity(
                entity, entity_id,
            )),
            targets: None,
        }
    }
}

#[derive(Debug)]
pub struct IntentManager {
    /// the start of this queue posses the intent with the highest priority
    queue: Vec<Intent>,
}

impl IntentManager {
    pub fn new() -> Self {
        IntentManager { queue: vec![] }
    }
    /// puts a new intent in the Queue, sorting it beforehand
    /// a new intent will be treated after older ones
    /// TODO : optimize this...
    pub fn submit(
        &mut self,
        intent: Intent,
    ) {
        // sort this new intent in the queue
        self.queue.insert(0, intent);
        // reaction to this intent with the same priority will be treated after this intent
        self.queue.sort_by(|a, b| (a).priority.cmp(&b.priority));
    }

    /// Extract precisely one smallest step of the next intent in queue
    /// used to realise your intent bit by bit transforming it into an action
    /// until the intent is exhausted
    /// or a higher priority takes its place
    /// when that happens,
    ///     puts back the rest of intent in the queue
    ///     return the small intent
    /// fails if queue empty but this should be unreachable
    pub fn extract_top_intent(&mut self) -> Result<Intent, QueueEmptyReached> {
        let max_priority_intent = self.queue.pop().ok_or(QueueEmptyReached {})?;
        let (minimal_intent, remainder_intent) = max_priority_intent.extract_minimal_intent();
        if let Some(x) = remainder_intent {
            self.queue.push(x);
        };
        Ok(minimal_intent)
    }

    pub fn is_queue_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

impl Default for IntentManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;
