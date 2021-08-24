use std::collections::HashMap;

use crate::{on_the_map::*, DijkstraMap, EntityId};
use dijkstra_map::Cost;
use fnv::{FnvHashMap, FnvHashSet};
// pub mod djikstra;
pub use dijkstra_map::Vector2D;
pub type Pos2D = Vector2D<i32, i32>;

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

pub struct Map {
    /// intern dijkstramap
    dijkstra_map: DijkstraMap,
    /// pos to dijkstraPointId
    pos_to_dijkstra_point_id: FnvHashMap<Pos2D, dijkstra_map::PointId>,
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
        Map {
            dijkstra_map,
            pos_to_dijkstra_point_id,
        }
    }
    /// returns a bool according to wether adding an entity at pos is possible
    pub fn can_entity_be_accepted_at_pos(&self, position: Pos2D) -> bool {
        todo!()
    }
    /// adds an entity on the map
    pub fn register_entity_at_pos(&mut self, entity_id: EntityId, position: Pos2D) {
        todo!()
    }
    /// return a
    /// ? list of path to get to reachable position excluding the first position where the entity is standing
    /// ? list of path to get to reachable position
    /// ? list of reachable position
    pub fn get_valid_movement_for_entity_at_pos(&self, entity: &Entity, position: Pos2D) {
        self.dijkstra_map.recalculate(
            &[*self.pos_to_dijkstra_point_id.get(&position).unwrap()],
            None,
            Some(Cost(entity.get_move_force())),
            Vec::new(),
            entity.terrain_weights(),
            FnvHashSet::default(),
        );
        // ? TODO : implement a get all points available from the djikstra_map side
        // ? (all point excluding the infinitly costing ones)
        // currently,
        let points_available = self
            .dijkstra_map
            .get_all_points_with_cost_between(Cost(0f32), Cost(entity.get_move_force()))
            .iter()
            .map(|x| self.dijkstra_map.get_shortest_path_from_point(*x).collect());
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
