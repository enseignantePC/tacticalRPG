//! This is the representation of the type of terrain
//! from the side of the game_manager
//!
//! Note : this requires a lot of dumb code and makes me unsure how usefull it is
//! to decouple entirerly things from djikstra crate
//!
//! Note : this could be simplified a lot by use of a macro (if ever someone feels up to it?)

use std::convert::TryFrom;

use super::*;
use gdnative::prelude::ToVariant;
#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy, ToVariant)]
/// representation of the type of terrain from the side of the game_manager
/// it will alter the movement of an entity sitting on it
///
pub enum TerrainType {
    Ground,
    Forest,
    Wall,
    Void,
    Water,
    Sky,
}

impl Into<i32> for &TerrainType {
    fn into(self) -> i32 {
        match self {
            TerrainType::Ground => 0,
            TerrainType::Forest => 1,
            TerrainType::Wall => 2,
            TerrainType::Void => 3,
            TerrainType::Water => 4,
            TerrainType::Sky => 5,
        }
    }
}

impl TryFrom<i32> for TerrainType {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, ()> {
        match value {
            1 => Ok(TerrainType::Ground),
            2 => Ok(TerrainType::Forest),
            3 => Ok(TerrainType::Wall),
            4 => Ok(TerrainType::Void),
            5 => Ok(TerrainType::Water),
            _ => Err(()),
        }
    }
}

impl Into<String> for TerrainType {
    fn into(self) -> String {
        match self {
            TerrainType::Ground => "Ground".to_string(),
            TerrainType::Forest => "Forest".to_string(),
            TerrainType::Wall => "Wall".to_string(),
            TerrainType::Void => "Void".to_string(),
            TerrainType::Water => "Water".to_string(),
            TerrainType::Sky => "Sky".to_string(),
        }
    }
}
// TODO : TESTME
/// This is glue code for mapping the terrain_weights member of an [Entity] to the terrain_weights arg
/// expected by the [DijkstraMap].
pub fn terrain_weights_to_dijkstra_terrain_weigth(
    x: &HashMap<terrains::TerrainType, f32>,
) -> FnvHashMap<dijkstra_map::TerrainType, dijkstra_map::Weight> {
    let mut result: FnvHashMap<dijkstra_map::TerrainType, dijkstra_map::Weight> =
        FnvHashMap::default();
    for (terrain, weight) in x {
        let dji_terrain_type = dijkstra_map::TerrainType::Terrain(terrain.into());
        let dji_weigth = dijkstra_map::Weight(*weight);
        result.insert(dji_terrain_type, dji_weigth);
    }
    result
}
