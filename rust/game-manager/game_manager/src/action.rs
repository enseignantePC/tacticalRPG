use crate::map::Pos2D;

/// Represents the options an entity can chose from for deciding what to do in the game
///
/// generated at with
///     entity intern state
///     external context

/// contains exhaustive info about a movement
#[derive(Clone, Debug)]
pub struct Move {
    /// every position the entity will cross to get to their final point,
    ///     starting with where they are now
    ///     ending where they will end
    path: Vec<Pos2D>,
}
/// contains exhaustive info about an attack  
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct Attack {
    pub _type: AttackType,
    pub strength: f64,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub enum AttackType {
    Base,
}

/// contains exhaustive info about an object use
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct Object {}
/// contains exhaustive info about a spell use
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct Spell {}
/// represents every kind of action an entity can do
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub enum ActionKind {
    Attack,
    Move,
    Object,
    Spell,
}
/// represents everything doable by entities
#[derive(Debug, Clone)]
pub enum Action {
    Attack(Attack),
    Move(Move),
    Object(Object),
    Spell(Spell),
}

impl Action {
    /// return an action devoid of sense most of the time for testing purposes
    pub fn void_action() -> Action {
        Action::Move(Move {
            path: vec![Pos2D(0, 0)],
        })
    }
}
#[derive(Debug)]
pub struct ActionManager {}

impl ActionManager {
    pub fn resolve(&self, action: Action) {
        todo!()
    }
}
