use crate::map::Pos2D;

/// Represents the options an entity can chose from for deciding what to do in the game
///
/// generated at with
///     entity intern state
///     external context

/// contains exhaustive info about a movement
#[derive(Clone)]
pub struct Move {
    /// every position the entity will cross to get to their final point,
    ///     starting with where they are now
    ///     ending where they will end
    path: Vec<Pos2D>,
}
/// contains exhaustive info about an attack  
#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct Attack {
    pub _type: AttackType,
    pub strength: f64,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum AttackType {
    Base,
}

/// contains exhaustive info about an object use
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Object {}
/// contains exhaustive info about a spell use
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Spell {}
/// represents every kind of action an entity can do
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum ActionKind {
    Attack,
    Move,
    Object,
    Spell,
}
/// represents everything doable by entities
pub enum Action {
    Attack(Attack),
    Move(Move),
    Object(Object),
    Spell(Spell),
}
