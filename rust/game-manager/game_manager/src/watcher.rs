use super::*;

/// A Watcher structure
/// - Accept subscriptions of EventWatchers
/// - It can be given an intent
/// (typically because it will be or has been sent
/// to the world to be resolved into consequences).
/// It returns itself a Vec of Intentions
/// that are the consequences wanted by the EventWatchers
/// 
/// Issuing Consequences has Side effect(?) as an entity might not
/// be capable of watching the same events multiple times?
pub struct Watcher {}

impl Watcher {
    /// TODO
    /// be capable of reacting (submitting an intent) that depends on the intent watched
    #[deprecated]
    pub fn watch(&mut self, next_intent: &Intent) -> Vec<Intent> {
        todo!();
        Vec::new()
    }
}
