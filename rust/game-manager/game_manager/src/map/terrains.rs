use super::*;
#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub enum TerrainType {
    Ground,
    Forest,
    Wall,
    Void,
    Water,
    Sky,
}

impl Into<i32> for TerrainType {
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

pub fn terrain_weights_to_dijkstra_terrain_weigth(
    x: &HashMap<terrains::TerrainType, i32>,
) -> FnvHashMap<dijkstra_map::TerrainType, dijkstra_map::Weight> {
    todo!()
}
