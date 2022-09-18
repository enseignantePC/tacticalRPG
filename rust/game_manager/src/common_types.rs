#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TerrainId(pub i32);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EntityId(pub i32);

#[derive(Debug, Clone)]
pub struct Selector;

#[derive(Debug, Clone)]
pub struct SelectorResult;

#[derive(Debug, Clone)]
pub struct Position;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Terrain {
    pub name: String,
    pub id: TerrainId,
    pub mode: TerrainMode,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum TerrainMode {
    EntityCanCross,
    ActionCanCross,
    NoneCanCross,
    // CustomCanCross(Data)
}

#[derive(Debug, Clone)]
pub struct Intent {
    emitter: EntityId,
    targets: Vec<EntityId>,
    action: Action,
}

#[derive(Debug, Clone)]
pub enum Action {
    Move,
    // signals an attack, a state change etc
    Message,
    PlaceWatcher,
    // Custom,
}

#[derive(Debug, Clone)]
pub enum WorldChange<EntityStateChangeData> {
    EntityMoved,
    EntityStateChanged(EntityStateChangeData),
    EntityPlaced,
    EntityUnplaced,
}

pub trait Entity {
    // the entity can receive these kinda message and have to update their internal state when they do
    type Message: Clone;
    // returned by the entity to tell the world what kinda consequence it got
    type EntityChange: Clone;

    fn get_play_options(&self) -> Vec<(Selector, Action)>;
    fn can_still_play(&self) -> bool;
    fn get_initiative(&self) -> f32;
    fn turn_finished(&mut self);
    fn get_message(
        &mut self,
        msg: Self::Message,
    ) -> WorldChange<Self::EntityChange>;
}

pub trait Watcher {
    fn watch(
        &mut self,
        intent: Intent,
    ) -> Intent;
}
