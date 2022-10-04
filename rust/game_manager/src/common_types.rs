use dijkstra_map::grids::Vector2D;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TerrainId(pub i32);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EntityId(pub i32);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum TeamId {
    Loner,
    Team(i32),
}
impl TeamId {
    pub fn is_ally(
        &self,
        oth: &TeamId,
    ) -> bool {
        if let TeamId::Team(x) = self {
            if let TeamId::Team(y) = oth {
                return *x == *y;
            }
        }
        false
    }
}

pub mod selector;
pub use selector::{Selector, SelectorResult,SelectorMode};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position(pub Vector2D<i32, i32>);

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
pub enum WorldChange<T: Entity> {
    EntityMoved {
        id: EntityId,
        from: Position,
        to: Position,
    },
    EntitySentMessage {
        from: EntityId,
        to: EntityId,
        msg: T::Message,
    },
    EntityStateChanged {
        id: EntityId,
        change: T::EntityChange,
    },
    EntityPlaced(EntityId, Position),
    EntityUnplaced(EntityId),
}

pub trait Entity: Sized + Clone {
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
    ) -> WorldChange<Self>;
}

pub trait Watcher {
    fn watch(
        &mut self,
        intent: Intent,
    ) -> Intent;
}
