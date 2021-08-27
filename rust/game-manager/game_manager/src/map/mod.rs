/// spatial representation of the world
///
/// holds the information of :
///     how is the world
///     what is it made of
///     who is where
/// what each case of the world is made of
use std::collections::HashMap;

use crate::{on_the_map::*, DijkstraMap, EntityId};
use dijkstra_map::{Cost, PointId};
use fnv::{FnvHashMap, FnvHashSet};
// pub mod djikstra;

pub use dijkstra_map::Vector2D;
pub type Pos2D = Vector2D<i32, i32>;

pub mod terrains;
use terrains::*;

/// everything interactable that can be in the world
pub enum Occupant {
    Vacant,
    Entity(Entity),
    Obstacle(Obstacle),
    Object(Object),
}

pub struct Map {
    /// intern dijkstramap
    dijkstra_map: DijkstraMap,
    /// pos to dijkstraPointId
    pos_to_dijkstra_point_id: FnvHashMap<Pos2D, dijkstra_map::PointId>,
    dijkstra_point_id_to_pos: FnvHashMap<dijkstra_map::PointId, Pos2D>,
    /// TODO : wire this up
    entity_id_to_pos: FnvHashMap<EntityId, Pos2D>,
    entity_pos_to_id: FnvHashMap<Pos2D, EntityId>,
    //  position -> TerrainType
    // interactable_map: FnvHashMap<Pos2D, TerrainType>,
    // position -> who or what is there
    // used to complement djikstramap result for coherent result with entities present on the map
}

impl Map {
    pub fn new(width: usize, height: usize) -> Map {
        let mut dijkstra_map = DijkstraMap::new();
        let pos_to_dijkstra_point_id = dijkstra_map.add_square_grid(
            width,
            height,
            None,
            dijkstra_map::TerrainType::DefaultTerrain,
            None,
            None,
        );
        let mut dijkstra_point_id_to_pos: FnvHashMap<dijkstra_map::PointId, Pos2D> =
            FnvHashMap::default();
        for ele in &pos_to_dijkstra_point_id {
            let (x, y) = ele.clone();
            dijkstra_point_id_to_pos.insert(*y, *x);
        }
        Map {
            dijkstra_map,
            pos_to_dijkstra_point_id,
            dijkstra_point_id_to_pos,
            entity_id_to_pos: FnvHashMap::default(),
            entity_pos_to_id: FnvHashMap::default(),
        }
    }
    /// returns a bool according to wether adding an entity at pos is possible
    pub fn can_entity_be_accepted_at_pos(&self, position: &Pos2D) -> bool {
        self.entity_pos_to_id.contains_key(position)
    }
    /// adds an entity on the map
    /// TODO : add a force option to put the entity on the map (by destroying whats there? by finding the closest place where the entity can go?)
    pub fn register_entity_at_pos(&mut self, entity_id: EntityId, position: &Pos2D) {
        todo!()
    }
    /// return a
    /// TODO : TESTME
    /// ? list of path to get to reachable position excluding the first position where the entity is standing
    /// ? list of path to get to reachable position
    /// ? list of reachable position
    pub fn get_valid_movements_for_entity(&mut self, entity: &Entity) -> Vec<Vec<Pos2D>> {
        self.recalculates_dijkstra_map_for_entity_with_force(entity, entity.get_move_force());
        // ? TODO : implement a get all points available from the djikstra_map side
        // ? (all point excluding the infinitly costing ones)
        // all points you can get to
        let end_points_available = self
            .dijkstra_map
            .get_all_points_with_cost_between(Cost(0f32), Cost(entity.get_move_force()));

        self.end_points_ids_to_paths_to_end_points(end_points_available)

        // .iter()
        // .map(|x| self.dijkstra_map.get_shortest_path_from_point(*x))
        // // .map(|x|x.collect::<>());
        // .collect();
    }
    /// currently, this methods returns what entity can be attacked by another entity
    pub fn get_attackable_entities_by_entity(&mut self, entity: &Entity) -> Vec<EntityId> {
        let result: Vec<EntityId> = Vec::new();
        for this_range in entity.get_attack_ranges() {
            self.recalculates_dijkstra_map_for_entity_with_force(entity, *this_range);
            let end_points_available = self
                .dijkstra_map
                .get_all_points_with_cost_between(Cost(0f32), Cost(entity.get_move_force()));
        }
        todo!()
    }

    fn recalculates_dijkstra_map_for_entity_with_force(&mut self, entity: &Entity, force: f32) {
        let position = self.entity_id_to_pos.get(&entity.unique_id).unwrap();
        self.dijkstra_map.recalculate(
            &[*self.pos_to_dijkstra_point_id.get(position).unwrap()],
            None,
            Some(Cost(force)),
            Vec::new(),
            terrains::terrain_weights_to_dijkstra_terrain_weigth(&entity.terrain_weights),
            FnvHashSet::default(),
        );
    }
    // ! if the dji map hasnt been precalculated, this returns empty array
    /// the paths returned doesnt include the point at which the entity is
    fn end_points_ids_to_paths_to_end_points(
        &self,
        end_points_available: &[PointId],
    ) -> Vec<Vec<Pos2D>> {
        let mut paths: Vec<Vec<Pos2D>> = Vec::new();

        for ele in end_points_available {
            let i = self.dijkstra_map.get_shortest_path_from_point(*ele);
            let mut v: Vec<Pos2D> = Vec::new();

            for yele in i {
                // turn pointiD to pos
                let pos = self.dijkstra_point_id_to_pos.get(&yele).unwrap();
                // push pos
                v.push(*pos)
            }
            paths.push(v);
        }
        paths
    }

    pub fn get_pos_for_entity(&self, id: EntityId) -> Option<Pos2D> {
        self.entity_id_to_pos.get(&id).map(|x| x.clone())
    }
    // pub fn print_terrain(&self) {
    //     // Je vais parcourir les positions,
    //     // chaque position en x est display à la suite
    //     // pour chaque ligne on print
    //     // - block_width points séparé par un espace
    //     // - centered Name of terrain
    //     // - block_width points séparé par un espace
    //     // 2 retour à la ligne
    //     let block_width = 10;
    //     let _return = 2;
    //     let size = todo!();
    //     let mut res = String::new();
    //     for y in 0..size.width {
    //         let mut l1 = String::new();
    //         let mut l2 = String::new();

    //         for x in 0..size.height {
    //             let pos = Pos2D::new(x as i32, y as i32);
    //             let terrain = self
    //                 .dijkstra_map
    //                 .get_terrain_for_point(*self.pos_to_id.get(&pos).unwrap())
    //                 .unwrap();
    //             let terrain_name =
    //                 terrain_type::terrain_name(terrain).expect("Unauthorized Terrain");
    //             l1 = ".".repeat(block_width);
    //             l1.push_str(&" ".repeat(2));

    //             l2 = String::new();
    //             l2.push_str(&format!("{:.^.width$}", terrain_name, width = block_width));
    //         }

    //         println!("{}", l1);
    //         println!("{}", l2);
    //         println!("{}", l1);
    //     }
    // }
}
