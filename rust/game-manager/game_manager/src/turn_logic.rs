use super::*;
/// represent the intent of doing something in the world
pub struct Intent {
    action: Action,
    priority: f64,
}

pub enum PlayOptions {
    Pass,
    ActionKind,
}

/// representation of what an entity wants to do

impl Intent {
    /// splits the intent into the more little Action and the rest of the intented action as an intent remainder
    pub fn poll_minimal_action(&self) -> (Intent, Action) {
        let minimal_action: Action = todo!(); 
        //self.action.minimal_action();
        let intent_remainder: Intent = todo!();
        (intent_remainder, minimal_action)
    }
}

/// you submit an intent
///
/// it is being sorted by priority (if equality,, first arrived first served)
///
/// then the intent is "resolved" : transformed into a real action with consequences in the world
///
/// between theses steps, entities (and maybe elements of nature) watch your intents and submit their own intent as a reaction
pub struct IntentSubmitter {
    queue: Vec<Intent>,
}
#[allow(unreachable_code)]
impl IntentSubmitter {
    /// puts a new intent in the Queue, sorting it beforehand
    pub fn submit(&self, intent: Intent) {
        // sort this new intent in the queue
        todo!()
    }

    /// declares your intent bit by bit transforming it into an action
    /// until the intent is epuised
    /// or a higher priority takes its place
    /// when that happens,
    ///     puts back the rest of intent in the queue
    ///     return the contigeous action
    fn try_resolve_top_intent(&self) -> Result<Action, ()> {
        let top_intent: Intent = todo!(); //pop the head of the queue
                                          // is top intent still faisable?

        let (intent, minimal_action) = top_intent.poll_minimal_action();
        /// submit minimal action
        todo!()
    }
}
