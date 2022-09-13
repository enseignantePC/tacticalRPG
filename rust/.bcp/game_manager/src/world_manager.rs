//! # The world manager module
//!
//! It is responsible for accepting [WorldChange]s
//! (a representation of what happened to the world) and
//! - Updating the state of the game accordingly
//! (for instance moving an entity around on the map) after it was decided the entity should move.
//! - Storing them (this is the built in implementation of an history)

use std::rc::Rc;

use crate::{
    attack_solver::ResolvedAttack,
    map::{terrains::Terrain, Pos2D},
};

use super::*;

/// this represents a changement in the global state, it will be
/// - stored (for historic features)
/// - send to an exterior source (that will handle animation etc)
#[derive(Debug, Clone, ToVariant)]
pub enum WorldChange {
    Attack(ResolvedAttack),
    Death(EntityId),
    TerrainChange(Pos2D, Terrain),
    EntityMoved(EntityId, Pos2D),
}

// TODO : what entities (by id) are affected and how, are they dead/out of the map? how they moved
// TODO : how the terrain is affected
// TODO : what object is used, what spell is launched etc

/// the function responsible for generating [WorldChange]s and applying them to the world
///
/// while the treatment of the intent can be randomise i.e. (chance of critics etc)
/// the WorldChange is not i.e. (it asserts there was a critic blow or not)
///
/// given an intent, it generates a WorldChange : the description of a change applied to the world
pub fn intent_to_world_change(
    entity_id_to_entity: &HashMap<EntityId, Rc<Entity>>,
    intent: Intent,
) -> Vec<WorldChange> {
    match intent.action {
        Action::Attack(a) => {
            let result = vec![WorldChange::Attack(
                attack_solver::solve(
                    entity_id_to_entity,
                    a,
                    intent.entity,
                ),
            )];
            result
        }
        Action::Move(m) => {
            let mut result = Vec::new();
            let last = m.path.last().expect("Couldn't get the last Pos of Entity");
            result.push(
                WorldChange::EntityMoved(
                    intent.entity.unique_id,
                    *last,
                ),
            );
            result
        }
        Action::Object(_) => todo!(),
        Action::Spell(_) => todo!(),
    }
}

/// resolve what effectively happens on the world and has an event system to trigger new intents to be sent according to what happened
///     a simple example would be: if someone attacks player A, player A always counter attacks
///     somehow more complex : if someone attacks player A and player A is in range of attacking, player A counter attacks
pub fn apply_change_to_world(
    change: &WorldChange,
    _game_manager: &mut GameManager,
) {
    match change {
        WorldChange::Attack(_) => {
            // get both entity and change their states
            todo!();
        }
        WorldChange::Death(_) => {
            // remove the entity from the map
            todo!();
        }
        WorldChange::TerrainChange(_, _) => todo!(),
        WorldChange::EntityMoved(_, _) => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_change_to_world_attack() {
        todo!()
    }
    #[test]
    fn test_apply_change_to_world_move() {
        todo!()
    }
    #[test]
    fn test_apply_change_to_world_death() {
        todo!()
    }
    #[test]
    fn test_apply_change_to_world_terrain_change() {
        todo!()
    }
}
