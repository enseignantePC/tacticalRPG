use std::collections::HashMap;

use crate::{on_the_map::*, DijkstraMap, EntityId};
use fnv::FnvHashMap;
// pub mod djikstra;
#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub struct Pos2D(pub i64, pub i64);
/// spatial representation of the world
///
/// holds the information of :
///     how is the world
///     what is it made of
///     who is where
/// what each case of the world is made of
#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub enum TerrainType {
    Forest,
    Ground,
    Wall,
    Void,
    Water,
    Sky,
    ByDefault,
}
/// everything interactable that can be in the world
pub enum Occupant {
    Vacant,
    Entity(Entity),
    Obstacle(Obstacle),
    Object(Object),
}

pub struct UninitialisedMap {}
pub struct Map {
    /// intern dijkstramap
    dijkstra_map: DijkstraMap,
    /// pos to dijkstraPointId
    pos_to_dijkstra_point_id: FnvHashMap<dijkstra_map::Vector2D<i32, i32>, dijkstra_map::PointId>,
    //  position -> TerrainType
    // interactable_map: FnvHashMap<Pos2D, TerrainType>,
    // position -> who or what is there
    // used to complement djikstramap result for coherent result with entities present on the map
}

impl Map {
    pub fn new() -> UninitialisedMap {
        UninitialisedMap {}
    }
    /// returns a bool according to wether adding an entity at pos is possible
    pub fn can_entity_be_accepted_at_pos(&self, position: Pos2D) -> bool {
        todo!()
    }
    /// adds an entity on the map
    pub fn register_entity_at_pos(&mut self, entity_id: EntityId, position: Pos2D) {
        todo!()
    }
}

impl UninitialisedMap {
    pub fn initialise(self, width: usize, height: usize) -> Map {
        let mut dijkstra_map = DijkstraMap::new();
        let pos_to_dijkstra_point_id = dijkstra_map.add_square_grid(
            width,
            height,
            None,
            dijkstra_map::TerrainType::DefaultTerrain,
            None,
            None,
        );
        Map {
            dijkstra_map,
            pos_to_dijkstra_point_id,
        }
    }
}
