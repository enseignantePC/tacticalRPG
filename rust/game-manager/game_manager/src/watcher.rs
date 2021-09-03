use super::*;
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
