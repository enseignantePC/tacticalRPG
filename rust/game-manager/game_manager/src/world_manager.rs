use super::*;

pub struct WorldChange {}

#[derive(Debug)]
pub struct WorldManager {}

impl WorldManager {
    /// given an intent, it `applies` it to the world, generating a WorldChange :
    /// description of a change applied to the world
    pub fn resolve(&self, intent: Intent) {
        todo!()
    }
}
