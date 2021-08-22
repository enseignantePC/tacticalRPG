use std::collections::HashMap;

use crate::{on_the_map::*, DijkstraMap};
use fnv::FnvHashMap;
// pub mod djikstra;

pub struct Pos2D(f64, f64);
/// spatial representation of the world
///
/// holds the information of :
///     how is the world
///     what is it made of
///     who is where
/// what each case of the world is made of
#[derive(Eq, Hash, PartialEq, Debug)]
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
pub struct Map {
    ///  position -> TerrainType
    dijkstra_map: DijkstraMap,
    /// pos to dijkstraPointId
    pos_to_dijkstraPointId: FnvHashMap<Vector2D<i32, i32>, PointId>,
    /// position -> who or what is there
    /// used to complement djikstramap result for coherent result with entities present on the map
    interactable_map: FnvHashMap<Pos2D, TerrainType>,
    //djikstra_map? en interne?
}

impl Map {
    fn setup_map(width: usize, height: usize) {
        let mut d = DijkstraMap::new();
        let dico = d.add_square_grid(
            width,
            height,
            None,
            dijkstra_map::TerrainType::DefaultTerrain,
            None,
            None,
        );
        (d, dico)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
