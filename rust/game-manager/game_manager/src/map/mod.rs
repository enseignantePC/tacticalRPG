//! # Module map
//!
//! This module is responsible for representing the world as a 2D grid
//! and computing/keeping track of everything that happens, grid wise
//! holding information about :
//! - the form of the world
//! - what is it made of (TerrainType)
//! - who is where
//! - what each case of the world is made of
//!
//! It uses a [DijkstraMap] to do the calculation and abstracts it so it can communicate
//! with a [GameManager].
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{on_the_map::*, DijkstraMap, EntityId, TeamID};
use dijkstra_map::{Cost, PointId};
use fnv::{FnvHashMap, FnvHashSet};
// pub mod djikstra;

pub use dijkstra_map::Vector2D;
pub type Pos2D = Vector2D<i32, i32>;

pub mod terrains;
use terrains::*;

#[derive(Debug, PartialEq, Eq, Hash)]
/// everything interactable that can be in the world and will be stored by the map
pub enum Occupant {
    Entity(EntityId),
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
    pos_to_occupant: FnvHashMap<Pos2D, Occupant>,
    /// for each team, the set of position taken by the team
    team_id_to_set_of_position_taken: FnvHashMap<TeamID, FnvHashSet<Pos2D>>,
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
            pos_to_occupant: FnvHashMap::default(),
            team_id_to_set_of_position_taken: FnvHashMap::default(),
        }
    }
    /// returns a bool according to wether adding an entity at pos is possible
    pub fn can_entity_be_accepted_at_pos(&self, position: &Pos2D) -> bool {
        self.pos_to_occupant.contains_key(position)
    }

    pub fn get_pos_for_entity(&self, id: EntityId) -> Option<Pos2D> {
        self.entity_id_to_pos.get(&id).map(|x| x.clone())
    }

    /// adds an entity on the map
    /// TODO : add a force option to put the entity on the map (by destroying whats there? by finding the closest place where the entity can go?)
    pub fn register_entity_at_pos(&mut self, entity: Rc<Entity>, position: &Pos2D) {
        let team = entity.team.clone();
        let id = entity.unique_id.clone();
        self.entity_id_to_pos.insert(id, position.clone());
        self.pos_to_occupant
            .insert(position.clone(), Occupant::Entity(id));

        if !self.team_id_to_set_of_position_taken.contains_key(&team) {
            self.team_id_to_set_of_position_taken
                .insert(team, FnvHashSet::default());
        }

        self.team_id_to_set_of_position_taken
            .get_mut(&team)
            .unwrap()
            .insert(position.clone());

        todo!()
    }
    /// Computes where an entity might go by what path and return the path in the form of
    /// a list of path to get to reachable position excluding the first position where the entity is standing.
    ///
    /// TODO : TESTME - disable points are the right ones?
    ///
    ///
    /// TODO : the map has to keep track of who's where and who's and who's team
    /// TODO : as a list of position easily updatable so it can forbid these position in the movement either
    /// - as end points if entities are on the same team
    /// - as end points and travel points, if [Occupant] cannot be crossed

    pub fn get_valid_movements_for_entity(&mut self, entity: &Entity) -> Vec<Vec<Pos2D>> {
        //store all points belonging to other teams
        // if loner, adds all point belonging to team except pos of entity

        let points_that_cannot_be_crossed = self.get_uncrossable_points_for_entity(entity);
        self.enable_all_djikstra_points();

        for k in &points_that_cannot_be_crossed {
            self.dijkstra_map.disable_point(*k).unwrap();
        }

        self.recalculates_dijkstra_map_for_entity_with_force(
            entity,
            entity.get_move_force(),
            entity.terrain_weights.clone(),
        );

        let end_points_available = self.points_available_filters_end_position(entity);

        self.end_points_ids_to_paths_to_end_points(end_points_available.as_slice())
    }

    /// all points you can get to
    /// minus where ur teamates are
    /// TODO : test me
    fn points_available_filters_end_position(&mut self, entity: &Entity) -> Vec<PointId> {
        let end_points_available: Vec<PointId> = self
            .dijkstra_map
            .get_all_points_with_cost_between(Cost(0f32), Cost(entity.get_move_force()))
            .iter()
            .filter(|&x| {
                let x = self.dijkstra_point_id_to_pos.get(x).unwrap();
                self.team_id_to_set_of_position_taken
                    .get(&entity.team)
                    .unwrap()
                    .contains(x)
            })
            .map(|x| x.clone())
            .collect();
        end_points_available
    }

    /// this methods is used to determine what occupant might be attacked by a specified [Entity]
    /// TODO : currently this is broken and only returns what [Entity]s can be attacked
    pub fn get_attackable_entities_by_entity(&mut self, entity: &Entity) -> Vec<EntityId> {
        // all entities in range that are not on the same team
        let result: Vec<EntityId> = Vec::new();
        for this_range in entity.get_attack_ranges() {
            self.recalculates_dijkstra_map_for_entity_with_force(
                entity,
                *this_range as f32,
                todo!(),
            );
            let end_points_available = self
                .dijkstra_map
                .get_all_points_with_cost_between(Cost(0f32), Cost(entity.get_move_force()));
        }
        todo!()
    }

    fn enable_all_djikstra_points(&mut self) {
        for k in self.pos_to_dijkstra_point_id.values() {
            self.dijkstra_map.enable_point(*k).unwrap();
        }
    }

    /// the entity cannot :
    /// - cross a pos where an ennemy entity is
    /// this function shoud return a vector containing these innaccessible positions
    fn get_uncrossable_points_for_entity(&mut self, entity: &Entity) -> Vec<PointId> {
        let mut uncrossable_points: Vec<PointId> = Vec::new();

        for (team, set) in &self.team_id_to_set_of_position_taken {
            if (&entity.team != team) || (&entity.team == &TeamID::Loner) {
                for pos in set {
                    uncrossable_points.push(*self.pos_to_dijkstra_point_id.get(pos).unwrap())
                }
            }
        }
        uncrossable_points
    }
    /// given a `force`, rebakes the [DijkstraMap] for an entity
    ///
    fn recalculates_dijkstra_map_for_entity_with_force(
        &mut self,
        entity: &Entity,
        force: f32,
        terrain_weights: HashMap<TerrainType, f32>,
    ) {
        let position = self.entity_id_to_pos.get(&entity.unique_id).unwrap();
        self.dijkstra_map.recalculate(
            &[*self.pos_to_dijkstra_point_id.get(position).unwrap()],
            None,
            Some(Cost(force)),
            Vec::new(),
            terrains::terrain_weights_to_dijkstra_terrain_weigth(&terrain_weights),
            FnvHashSet::default(),
        );
    }
    /// ! if the dji map hasnt been precalculated, this returns empty array
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
