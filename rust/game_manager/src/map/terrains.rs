//! # The Map.terrain module
//!
//! It deals with different Terrains on which entity may
//! be forbidden to step. Or attack etc. and translation
//! from this concept to the intern [DijkstraMap]
//!
//! The TerrainManager will be used to
//! declare what [Terrain]s exist and what is their
//! [TerrainType] so the [GameManager][crate::GameManager]
//! can deal with them in a nice manner.

use super::*;
use bidir_map::BidirMap;
use gdnative::prelude::ToVariant;
use std::{
    convert::TryFrom,
    fmt::{write, Debug, Display},
};

/// This struct holds [Terrain] value and is meant to
/// be owned by a [GameManager][crate::GameManager]
#[derive(Debug)]
pub struct TerrainManager {
    max_used_id: i32,
    bi_map: BidirMap<i32, Terrain>,
}

impl TerrainManager {
    pub fn new() -> Self {
        Self {
            max_used_id: 0,
            bi_map: BidirMap::new(),
        }
    }

    fn new_id(&mut self) -> i32 {
        self.max_used_id += 1;
        self.max_used_id - 1
    }

    pub fn with_terrain_crossable(
        &mut self,
        name: &str,
    ) {
        self.new_terrain(
            name,
            TerrainType::EntityMayCross,
        );
    }
    pub fn with_terrain_attack_crossable(
        &mut self,
        name: &str,
    ) {
        self.new_terrain(
            name,
            TerrainType::AttackMayCross,
        );
    }
    pub fn with_terrain_uncrossable(
        &mut self,
        name: &str,
    ) {
        self.new_terrain(
            name,
            TerrainType::NoneMayCross,
        );
    }
    /// this returns a map where every terrain has a weight of one, so the
    /// attacks flings no matter the terrain except attacks through which
    /// the terrain cannot pass through (they have infinite weight).
    pub fn terrain_weight_for_attacks(&self) -> HashMap<Terrain, f32> {
        let mut result: HashMap<Terrain, f32> = HashMap::new();
        let attack_may_cross: HashMap<Terrain, f32> = self
            .bi_map
            .second_col()
            .into_iter()
            .filter(|&t| {
                !matches!(
                    t._type,
                    TerrainType::NoneMayCross,
                )
            })
            .map(|t| (t.clone(), 1f32))
            .collect();

        let attack_may_not_cross: HashMap<Terrain, f32> = self
            .bi_map
            .second_col()
            .into_iter()
            .filter(|&x| {
                matches!(
                    x._type,
                    TerrainType::NoneMayCross,
                )
            })
            .map(|t| {
                (
                    t.clone(),
                    f32::INFINITY,
                )
            })
            .collect();
        result.extend(attack_may_cross);
        result.extend(attack_may_not_cross);
        result
    }

    fn new_terrain(
        &mut self,
        name: &str,
        _type: TerrainType,
    ) {
        let id = self.new_id();
        let t = Terrain {
            unique_id: id,
            name: name.into(),
            _type,
        };
        self.bi_map
            .insert(id, t)
            .expect("tried to add an already existing terrain or id to the terrain manager");
    }
}

impl Default for TerrainManager {
    fn default() -> Self {
        Self::new()
    }
}

// TODO : TESTME
/// This is glue code for mapping the terrain_weights member
/// of an [Entity] to the terrain_weights arg
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

/// representation of the type of terrain from the side of the game_manager
/// it will alter the movement of an entity sitting on it
#[derive(Eq, Hash, PartialEq, Debug, Clone, ToVariant)]
pub struct Terrain {
    unique_id: i32,
    name: String,
    _type: TerrainType,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, ToVariant)]
pub enum TerrainType {
    NoneMayCross,
    AttackMayCross,
    EntityMayCross,
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
