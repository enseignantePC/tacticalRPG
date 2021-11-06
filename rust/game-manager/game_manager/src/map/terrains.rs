//! This is the representation of the type of terrain
//! from the side of the game_manager
//!
//! Note : this requires a lot of dumb code and makes me unsure how useful it is
//! to decouple entirely things from dijkstra crate
//!
//! Note : this could be simplified a lot by use of a macro (if ever someone feels up to it?)

use std::{
    convert::TryFrom,
    fmt::{write, Debug, Display},
};

use super::*;
use gdnative::prelude::ToVariant;

/// This struct holds possible terrain value
#[derive(Debug)]
pub struct TerrainManager {}

/// representation of the type of terrain from the side of the game_manager
/// it will alter the movement of an entity sitting on it
#[derive(Eq, Hash, PartialEq, Debug, Clone, ToVariant)]
pub struct Terrain {
    unique_id: i32,
    name: String,
    _type: TerrainType,
}

impl Terrain {
    pub(crate) fn new(
        unique_id: i32,
        name: String,
        _type: TerrainType,
    ) -> Self {
        Terrain {
            unique_id,
            name,
            _type,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, ToVariant)]
pub enum TerrainType {
    ImpossibleToCross,
    AttackMayCross,
    EntityMayCross,
}

impl From<&Terrain> for i32 {
    fn from(val: &Terrain) -> Self {
        val.unique_id
    }
}

impl Display for Terrain {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{} : {}",
            self.name, self.unique_id
        )
    }
}

// TODO : TESTME
/// This is glue code for mapping the terrain_weights member of an [Entity] to the terrain_weights arg
/// expected by the [DijkstraMap].
pub fn terrain_weights_to_dijkstra_terrain_weight(
    terrain_weight: &HashMap<terrains::Terrain, f32>
) -> FnvHashMap<dijkstra_map::TerrainType, dijkstra_map::Weight> {
    let mut result: FnvHashMap<dijkstra_map::TerrainType, dijkstra_map::Weight> =
        FnvHashMap::default();

    for (terrain, weight) in terrain_weight {
        let dji_terrain_type = dijkstra_map::TerrainType::Terrain(i32::from(terrain));
        let dji_weight = dijkstra_map::Weight(*weight);
        result.insert(
            dji_terrain_type,
            dji_weight,
        );
    }
    result
}

/// This maps is used to determine where and through what type of terrain can attack go
/// it is currently hardcoded that they go through anything except walls,
///
/// this could be the default while the entity concerned optionally provide correction to it
pub fn terrain_weight_for_attacks() -> HashMap<Terrain, f32> {
    let mut result: HashMap<Terrain, f32> = HashMap::new();

    // for terrain_type in todo!() {
    //     let weight = 1.0;
    //     result.insert(terrain_type, weight);
    // }
    // result.insert(terrain_type, weight);
    todo!();
    result
}
