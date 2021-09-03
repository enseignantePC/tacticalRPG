use std::rc::Rc;

use super::*;
/// represents the intention of an [Entity] to do any kind of [Action] in the world
#[derive(Debug, Clone)]
pub struct Intent {
    /// the action the entity wants to accomplish
    pub action: Action,
    /// an indicator that changes how fast the intent will be treated by the [IntentManager],
    /// higher priority means the intent will be treated faster.
    pub priority: i32,
    /// a reference to the entity that wants to do the action
    pub entity: Rc<Entity>,
}

/// representation of what an entity wants to do

impl Intent {
    /// split the intent in two :
    /// - the smallest part possible of the intent that should be use to immediatly change the world
    /// - the remainder of the intent, that should be put back on the intent queue, and treated before
    /// newer intent are submitted to the [IntentManager]
    ///
    /// TESTME : if the remainder path becomes empty, the remainder intent returned is None
    /// TESTME : the remainder intent is always smaller that the original intent
    pub fn extract_minimal_intent(self) -> (Intent, Option<Intent>) {
        match self.action {
            Action::Attack(a) => todo!(),

            Action::Move(mut m) => {
                let next_path = vec![m.path.remove(0)];
                let remainder_should_be_none = m.path.is_empty();
                let (minimal_intent, remainder_intent) = (
                    Intent {
                        action: Action::Move(Move { path: next_path }),
                        priority: self.priority,
                        entity: self.entity.clone(),
                    },
                    Intent {
                        action: Action::Move(Move { path: m.path }),
                        priority: self.priority,
                        entity: self.entity.clone(),
                    },
                );
                let remainder_intent = if remainder_should_be_none {
                    None
                } else {
                    Some(remainder_intent)
                };
                (minimal_intent, remainder_intent)
            }

            Action::Object(o) => todo!(),
            Action::Spell(s) => todo!(),
        }
    }
    // for test purposes
    // pub fn void_intent() -> Intent {
    //     Intent {
    //         action: Action::void_action(),
    //         priority: 0,
    //         entity: Rc::new(super::on_the_map::Entity::example_entity()),
    //     }
    // }
}

#[derive(Debug)]
pub struct IntentManager {
    /// the start of this queue posses the intent with the highest priority
    queue: Vec<Intent>,
}

impl IntentManager {
    /// puts a new intent in the Queue, sorting it beforehand
    /// a new intent will be treated after older ones
    /// TODO : optimize this...
    pub fn submit(&mut self, intent: Intent) {
        // sort this new intent in the queue
        self.queue.insert(0, intent.clone());
        // reaction to this intent with the same priority will be treated after this intent
        self.queue.sort_by(|a, b| (&a).priority.cmp(&b.priority));
    }

    /// Extract precisely one smallest step of the next intent in queue
    /// used to realise your intent bit by bit transforming it into an action
    /// until the intent is exhausted
    /// or a higher priority takes its place
    /// when that happens,
    ///     puts back the rest of intent in the queue
    ///     return the small intent
    /// fails if queue empty but this should be unreachable
    /// ! TESTME an intent partially treated should not start at the beginning at the queue again
    pub fn extract_top_intent(&mut self) -> Result<Intent, ()> {
        let max_priority_intent = self.queue.pop().ok_or(())?;
        let (minimal_intent, remainder_intent) = max_priority_intent.extract_minimal_intent();
        if remainder_intent.is_some() {
            self.queue.push(remainder_intent.unwrap());
        };
        Ok(minimal_intent)
    }

    pub fn is_queue_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

// #[cfg(test)]
// mod tests {
//     use std::vec;

//     use super::*;

//     #[test]
//     fn sort_intents_correctly() {
//         let a = ActionManager {};
//         let mut im = IntentManager { queue: vec![] };

//         let mut i = Intent::void_intent();
//         i.priority = 1;
//         im.submit(i.clone());

//         i.priority = 7;
//         im.submit(i.clone());

//         i.priority = 4;
//         im.submit(i.clone());

//         dbg!(im.queue.pop());
//         todo!()
//     }
//     #[test]
//     fn new_intents_gets_treated_last() {
//         todo!()
//     }
//     #[test]
//     fn remainder_intents_gets_treated_first() {
//         todo!()
//     }
//     #[cfg(test)]
//     mod extract_minimal_intent {
//         use crate::map::Pos2D;

//         use super::*;

//         #[test]
//         fn one() {
//             let intent = Intent {
//                 action: Action::Move(Move {
//                     path: vec![Pos2D::new(1, 2)],
//                 }),
//                 priority: 0,
//                 entity: Rc::new(super::Entity::example_entity()),
//             };
//             let (a, b) = intent.extract_minimal_intent();
//             if let Action::Move(x) = a.action {
//                 assert_eq!(x.path, vec![Pos2D::new(1, 2)]);
//             } else {
//                 assert!(false);
//             };
//             assert!(b.is_none());
//         }
//         #[test]
//         fn two() {
//             let intent = Intent {
//                 action: Action::Move(Move {
//                     path: vec![Pos2D::new(1, 2), Pos2D::new(3, 4), Pos2D::new(5, 6)],
//                 }),
//                 priority: 0,
//                 entity: Rc::new(super::Entity::example_entity()),
//             };
//             let (a, b) = intent.extract_minimal_intent();
//             if let Action::Move(x) = a.action {
//                 assert_eq!(x.path, vec![Pos2D::new(1, 2)]);
//             } else {
//                 assert!(false);
//             };
//             assert!(b.is_some());
//             let b = b.unwrap();
//             if let Action::Move(x) = b.action {
//                 assert_eq!(x.path, vec![Pos2D::new(3, 4), Pos2D::new(5, 6)]);
//             } else {
//                 assert!(false);
//             };
//         }
//         #[test]
//         fn priority_kept() {
//             let intent = Intent {
//                 action: Action::Move(Move {
//                     path: vec![Pos2D::new(1, 2), Pos2D::new(3, 4), Pos2D::new(5, 6)],
//                 }),
//                 priority: 4,
//                 entity: Rc::new(super::Entity::example_entity()),
//             };
//             let (a, b) = intent.extract_minimal_intent();
//             assert_eq!(a.priority, 4);
//             assert_eq!(
//                 b.expect("remainder intent not expected to be none")
//                     .priority,
//                 4
//             );
//             dbg!(a);
//         }
//     }
// }
