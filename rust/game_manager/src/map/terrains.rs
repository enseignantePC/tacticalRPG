use super::*;

pub struct TerrainSet {
    id_gen: i32,
    data: FnvHashMap<Terrain, dijkstra_map::TerrainType>,
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
            data: FnvHashMap::default(),
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
