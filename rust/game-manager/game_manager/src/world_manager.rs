use super::*;

#[derive(Debug, Clone)]
pub struct WorldChange {}

#[derive(Debug)]
pub struct WorldManager {}

impl WorldManager {
    /// given an intent, it generates a WorldChange :
    /// description of a change applied to the world
    /// while the treatment of the intent can be randomise i.e. (chance of critics etc)
    /// the WorldChange is not i.e. (it asserts there was a critic blow or not)
    pub fn intent_to_world_change(&self, intent: Intent) -> WorldChange {
        todo!()
    }
    pub fn apply_change_to_world(&self, change: &WorldChange, map: &mut map::Map) {
        todo!()
    }
}
