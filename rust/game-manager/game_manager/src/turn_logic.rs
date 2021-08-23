use super::*;
/// represent the intent of doing something in the world
#[derive(Debug, Clone)]
pub struct Intent {
    pub action: Action,
    priority: i64,
}

pub enum PlayOptions {
    Pass,
    ActionKind,
}

/// representation of what an entity wants to do

impl Intent {
    /// splits the intent into the more little Action and the rest of the intented action as an intent remainder
    pub fn extract_minimal_intent(&self) -> (Intent, Intent) {
        let minimal_intent: Intent = todo!();
        //self.action.minimal_action();
        let remainder_intent: Intent = todo!();
        (minimal_intent, remainder_intent)
    }
    pub fn void_intent() -> Intent {
        Intent {
            action: Action::void_action(),
            priority: 0,
        }
    }
}

/// you submit an intent
///
/// it is being sorted by priority (if equality,, first arrived first served)
///
/// then the intent is "resolved" : transformed into a real action with consequences in the world
///
/// between theses steps, entities (and maybe elements of nature) watch your intents and submit their own intent as a reaction
#[derive(Debug)]
pub struct IntentManager {
    /// the start of this queue posses the intent with the highest priority
    queue: Vec<Intent>,
}
#[allow(unreachable_code)]
impl IntentManager {
    /// puts a new intent in the Queue, sorting it beforehand
    /// a new intent will be treated after older ones
    /// TODO : optimize this...
    pub fn submit(&mut self, intent: Intent) {
        // sort this new intent in the queue
        self.queue.insert(0, intent);
        self.queue.sort_by(|a, b| (&a).priority.cmp(&b.priority))
    }
    /// Treat precisely one smallest step of the next intent in queue
    /// declares your intent bit by bit transforming it into an action
    /// until the intent is epuised
    /// or a higher priority takes its place
    /// when that happens,
    ///     puts back the rest of intent in the queue
    ///     return the small intent
    /// fails if queue empty
    /// ! TESTME an intent partially treated should not start at the beginning at the queue again
    pub fn resolve_one_intent(&mut self) -> Result<Intent, ()> {
        let max_priority_intent = self.queue.pop().ok_or(())?; // pops intent with highest priority
        let (minimal_intent, remainder_intent) = max_priority_intent.extract_minimal_intent();
        self.queue.push(remainder_intent);
        Ok(minimal_intent)
    }
    pub fn is_queue_empty(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn sort_intents_correctly() {
        let a = ActionManager {};
        let mut im = IntentManager { queue: vec![] };

        let mut i = Intent::void_intent();
        i.priority = 1;
        im.submit(i.clone());

        i.priority = 7;
        im.submit(i.clone());

        i.priority = 4;
        im.submit(i.clone());

        dbg!(im.queue.pop());
        todo!()
    }
    #[test]
    fn new_intents_gets_treated_last() {
        todo!()
    }
    #[test]
    fn remainder_intents_gets_treated_first() {
        todo!()
    }
}
