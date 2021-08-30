//! manages the states of the world, accept [WorldChange]s and
//! - Storing them
//! - updating the global state accordingly

use super::*;

/// this represents a changement in the global state, it will be
/// - stored (for historic features)
/// - send to an exterior source (that will handle animation etc)
#[derive(Debug, Clone)]
pub struct WorldChange {}

/// the structure responsible for generating [WorldChange]s and appling them to the world
// TODO : what entities (by id) are affected and how, are they dead/out of the map? how they moved
// TODO : how the terrain is affected
// TODO : what object is used, what spell is launched etc
#[derive(Debug, Clone)]
pub struct WorldManager {}

impl WorldManager {
    /// given an intent, it generates a WorldChange :
    /// description of a change applied to the world
    /// while the treatment of the intent can be randomise i.e. (chance of critics etc)
    /// the WorldChange is not i.e. (it asserts there was a critic blow or not)
    pub fn intent_to_world_change(&self, intent: Intent) -> WorldChange {
        todo!()
    }
    pub fn apply_change_to_world(&self, change: &WorldChange, gm: &mut GameManager) {
        todo!()
    }
}
