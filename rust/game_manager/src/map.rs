use dijkstra_map::DijkstraMap;

use std::collections::HashMap;

use crate::common_types::{
    Action, Intent, Selector, SelectorResult, Terrain, TerrainId, TerrainMode,
};

pub struct TerrainSet {
    id_gen: i32,
    data: HashMap<Terrain, dijkstra_map::TerrainType>,
}

pub struct Map {
    intern_map: DijkstraMap,
    terrains: TerrainSet,
}

impl Map {
    pub fn new(
        terrains: TerrainSet,
        size: (usize, usize),
    ) -> Self {
        let mut im = dijkstra_map::DijkstraMap::new();
        let h = im.add_square_grid(
            size.0,
            size.1,
            None,
            dijkstra_map::TerrainType::DefaultTerrain,
            None,
            None,
        );
        todo!("h unused");
        Map {
            intern_map: im,
            terrains,
        }
    }
    pub fn select(
        &self,
        selector: Selector,
    ) -> SelectorResult {
        todo!()
    }
}

impl SelectorResult {
    pub fn is_not_empty(&self) -> bool {
        todo!()
    }

    pub fn to_intent(
        self,
        action: Action,
    ) -> Intent {
        todo!()
    }
}

impl Terrain {
    pub fn new(
        name: String,
        id: TerrainId,
        mode: TerrainMode,
    ) -> Self {
        Terrain { name, id, mode }
    }
}

impl TerrainSet {
    fn get_next_id(&mut self) -> TerrainId {
        self.id_gen += 1;
        TerrainId(self.id_gen)
    }

    pub fn to_dijkstra_terrain(
        &self,
        terrain: Terrain,
    ) {
        todo!()
    }

    pub fn add_terrain(
        &mut self,
        terrain_name: &str,
        terrain_mode: TerrainMode,
        dijkstra_terrain: dijkstra_map::TerrainType,
    ) {
        let terrain = Terrain {
            name: terrain_name.into(),
            id: self.get_next_id(),
            mode: terrain_mode,
        };
        self.data.insert(
            terrain,
            dijkstra_terrain,
        );
    }

    pub fn new() -> Self {
        let mut set = TerrainSet {
            data: HashMap::new(),
            id_gen: 0,
        };
        set.add_terrain(
            "Default",
            TerrainMode::EntityCanCross,
            dijkstra_map::TerrainType::DefaultTerrain,
        );
        set
    }
}
