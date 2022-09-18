use bidir_map::BidirMap;
use dijkstra_map::{grids::Vector2D, DijkstraMap, PointId};

use std::hash::BuildHasherDefault;

use dijkstra_map::FnvHashMap;

use crate::common_types::{
    Action, EntityId, Intent, Position, Selector, SelectorResult, Terrain, TerrainId, TerrainMode,
};

pub mod terrains;
use terrains::TerrainSet;

pub struct Map {
    intern_map: DijkstraMap,
    terrains: TerrainSet,
    pos_to_point: FnvHashMap<Vector2D<i32, i32>, PointId>,
    entities_positions: BidirMap<EntityId, Position>,
    size: (usize, usize),
}

impl Map {
    pub fn new(
        terrains: TerrainSet,
        size: (usize, usize),
    ) -> Self {
        let mut im = dijkstra_map::DijkstraMap::new();
        let pos_to_point = im.add_square_grid(
            size.0,
            size.1,
            None,
            dijkstra_map::TerrainType::DefaultTerrain,
            None,
            None,
        );
        Map {
            intern_map: im,
            terrains,
            pos_to_point,
            entities_positions: BidirMap::new(),
            size,
        }
    }
    pub fn select(
        &self,
        selector: Selector,
    ) -> SelectorResult {
        todo!()
    }
    ///
    pub fn is_out_of_bounds(
        &self,
        pos: Position,
    ) -> bool {
        !self.pos_to_point.contains_key(&pos.0)
    }

    pub fn is_occupied(
        &self,
        pos: Position,
    ) -> bool {
        self.entities_positions.contains_second_key(&pos)
    }

    pub fn place(
        &mut self,
        entity: EntityId,
        pos: Position,
    ) {
        self.entities_positions.insert(entity, pos);
    }

    pub fn move_entity(
        id: EntityId,
        new_pos: Position,
    ) {
        todo!()
    }

    pub fn unplace(
        &mut self,
        e: EntityId,
    ) {
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
