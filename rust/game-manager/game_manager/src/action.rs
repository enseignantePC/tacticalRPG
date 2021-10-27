//! This module is responsible for offering a description of things
//! that entity can do while the game is playing
//! such things are called actions and represented by an [Action] structure.

use crate::map::Pos2D;
pub use crate::map::Vector2D;

/// Represents the options an entity can chose from for deciding what to do in the game
///
/// generated at with
///     entity intern state
///     external context
#[derive(PartialEq, Clone, Debug)]
pub enum Action {
    Attack(Attack),
    Move(Move),
    Object(Object),
    Spell(Spell),
}

/// contains exhaustive info about the movement of an entity, excluing the initial pos of the entity
#[derive(PartialEq, Clone, Debug)]
pub struct Move {
    /// every position the entity will cross to get to their final point,
    ///     excluing where they are now
    ///     ending where they will end
    pub path: Vec<Pos2D>,
}
/// contains exhaustive info about an attack  
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct Attack {
    pub _type: AttackType,
    pub strength: f64,
    pub crit_chance: f32,
}

/// This describes the type of attacks that exist in the game,
/// the logic being handled by the [attack solver][super::attack_solver]
///
/// Possible ideas are
/// spear /axe / sword like in FireEmblem or Shining Force
/// elemental / basic
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub enum AttackType {
    Base,
}

/// contains exhaustive info about an object use
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct Object {}
/// contains the info about a spell use
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct Spell {}
/// represents every kind of action an entity can do
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum ActionKind {
    Attack,
    Move,
    Object,
    Spell,
}

impl Action {
    /// return an action devoid of sense most of the time for testing purposes
    pub fn void_action() -> Action {
        Action::Move(Move {
            path: vec![Pos2D::new(0, 0)],
        })
    }
}
