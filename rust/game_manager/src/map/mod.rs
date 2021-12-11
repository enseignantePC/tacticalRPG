//! # the map module
//!
//! It provides the [Map] type that keeps information about :
//! - the form of the world, what is connected to what etc
//! - what is it made of (TerrainType)
//! - where each of the entity is.
//! - what each case of the world is made of.
//!
//! It uses a [map::Map] that wraps [DijkstraMap] to do the calculation and abstracts
//! it so it can communicate with a [GameManager].

use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
    rc::Rc,
};

use crate::{on_the_map::*, Action, DijkstraMap, EntityId, Intent, Move, PositionOccupied, TeamId};
use dijkstra_map::{Cost, PointId};
use fnv::{FnvHashMap, FnvHashSet};

pub mod terrains;
use terrains::*;

pub mod position;
pub use position::*;

pub mod select;
use select::*;

#[derive(Debug)]
pub struct Map {
    /// intern dijkstra_map
    dijkstra_map: DijkstraMap,
    /// pos to dijkstraPointId
    pos_to_dijkstra_point_id: FnvHashMap<Pos2D, dijkstra_map::PointId>,
    dijkstra_point_id_to_pos: FnvHashMap<dijkstra_map::PointId, Pos2D>,
    entity_id_to_pos: FnvHashMap<EntityId, Pos2D>,
    pos_to_occupant: FnvHashMap<Pos2D, Occupant>,
    /// for each team, the set of position taken by the team
    team_id_to_set_of_position_taken: FnvHashMap<TeamId, FnvHashSet<Pos2D>>,
}

impl Map {
    pub fn new(
        width: usize,
        height: usize,
    ) -> Map {
        let mut dijkstra_map = DijkstraMap::new();
        let pos_to_dijkstra_point_id = dijkstra_map.add_square_grid(
            width,
            height,
            None,
            dijkstra_map::TerrainType::DefaultTerrain,
            None,
            None,
        );
        // todo, give a better name
        let mut value: FnvHashMap<Pos2D, PointId> = FnvHashMap::default();
        for (k, v) in pos_to_dijkstra_point_id {
            value.insert(Pos2D(k), v);
        }
        let pos_to_dijkstra_point_id = value;
        let mut dijkstra_point_id_to_pos: FnvHashMap<dijkstra_map::PointId, Pos2D> =
            FnvHashMap::default();
        for ele in &pos_to_dijkstra_point_id {
            let (pos, dji_id) = ele;
            dijkstra_point_id_to_pos.insert(*dji_id, *pos);
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
    pub fn can_entity_be_accepted_at_pos(
        &self,
        position: &Pos2D,
    ) -> bool {
        self.pos_to_occupant.contains_key(position)
    }

    pub fn get_pos_for_entity(
        &self,
        id: EntityId,
    ) -> Option<Pos2D> {
        self.entity_id_to_pos.get(&id).copied()
    }

    /// adds an entity on the map
    /// fails if an entity is already at pos
    /// TODO : add a force option to put the entity on the map (by destroying whats there? by finding the closest place where the entity can go?)
    pub fn register_entity_at_pos(
        &mut self,
        entity: Rc<Entity>,
        position: &Pos2D,
    ) -> Result<(), PositionOccupied> {
        let team = entity.team;
        let id = entity.unique_id;
        if self.entity_id_to_pos.get(&id).is_some() {
            return Err(PositionOccupied);
        }

        self.entity_id_to_pos.insert(id, *position);
        self.pos_to_occupant.insert(
            *position,
            Occupant::Entity(entity),
        );

        self.team_id_to_set_of_position_taken
            .entry(team)
            .or_insert_with(FnvHashSet::default);

        self.team_id_to_set_of_position_taken
            .get_mut(&team)
            .unwrap()
            .insert(*position);
        Ok(())
    }
    #[allow(clippy::result_unit_err)]
    /// moves an entity to a new position, updating the maps internal accordingly.
    ///  fails if an entity is present on arrival
    pub fn move_entity_from_current_position_to_next_position(
        &mut self,
        entity: Rc<Entity>,
        next_position: Pos2D,
    ) -> Result<(), ()> {
        let id = entity.unique_id;
        let current_position = self
            .get_pos_for_entity(id)
            .expect("no position found for entity_id");

        self.entity_id_to_pos.remove_entry(&id);
        let res = self.entity_id_to_pos.insert(id, next_position);
        if res.is_some() {
            panic!("trying to move entity whom'st id is not on the map")
        }

        let (_, occupant) = self
            .pos_to_occupant
            .remove_entry(&current_position)
            .expect("trying to move entity who is not at pos");
        let res = self.pos_to_occupant.insert(
            next_position,
            occupant,
        );
        if res.is_some() {
            return Err(());
        }

        let set = self
            .team_id_to_set_of_position_taken
            .get_mut(&entity.team)
            .expect("no set found for team when it should have had");
        set.remove(&current_position);
        set.insert(next_position);
        Ok(())
    }

    pub fn remove_entity_from_the_map(
        &mut self,
        entity: Rc<Entity>,
    ) {
        let id = &entity.unique_id;
        let current_position = self
            .get_pos_for_entity(*id)
            .expect("no position found for entity_id");

        self.entity_id_to_pos.remove_entry(id);

        self.pos_to_occupant
            .remove_entry(&current_position)
            .expect("No occupant found at pos");

        let set = self
            .team_id_to_set_of_position_taken
            .get_mut(&entity.team)
            .expect("no set found for team when it should have had");
        set.remove(&current_position);
    }

    // TESTME
    pub fn get_valid_movements_for_entity(
        &mut self,
        entity: Rc<Entity>,
    ) -> Vec<Intent> {
        let mut result = Vec::new();
        for path in self.get_valid_paths_for_entity(entity.clone()) {
            let intent = Intent {
                action: Action::Move(Move { path }),
                // TODO : priority system
                priority: 0,
                entity: entity.clone(),
            };
            result.push(intent);
        }
        result
    }

    pub fn get_valid_attacks_for_entity(
        &self,
        _entity: Rc<Entity>,
    ) -> Vec<Intent> {
        todo!()
    }

    pub fn get_valid_object_for_entity(
        &self,
        _entity: Rc<Entity>,
    ) -> Vec<Intent> {
        todo!()
    }

    pub fn get_valid_spells_for_entity(
        &self,
        _entity: Rc<Entity>,
    ) -> Vec<Intent> {
        todo!()
    }

    /// Computes where an entity might go by what path and return the path in the form of
    /// a list of path to get to reachable position excluding the first position where the entity is standing.
    ///
    ///
    /// TODO : TESTME - disable points are the right ones?
    ///
    ///
    /// TODO : the map has to keep track of who's where and who's and who's team
    /// TODO : as a list of position easily updatable so it can forbid these position in the movement either
    /// - as end points if entities are on the same team
    /// - as end points and travel points, if [Occupant] cannot be crossed

    fn get_valid_paths_for_entity(
        &mut self,
        entity: Rc<Entity>,
    ) -> Vec<Vec<Pos2D>> {
        //store all points belonging to other teams
        // if loner, adds all point belonging to team except pos of entity

        let points_that_cannot_be_crossed = self.get_uncrossable_points_for_entity(entity.clone());
        self.enable_all_dijkstra_points();

        for k in &points_that_cannot_be_crossed {
            self.dijkstra_map.disable_point(*k).unwrap();
        }

        self.recalculates_dijkstra_map_for_entity_with_force(
            entity.clone(),
            entity.entity_intern.move_force(),
            entity.entity_intern.terrain_weights(),
        );

        let end_points_available = self.points_available_filters_end_position(entity);

        self.end_points_ids_to_paths_to_end_points(end_points_available.as_slice())
    }

    /// all points you can get to
    /// minus where ur teammates are
    /// TODO : test me
    fn points_available_filters_end_position(
        &mut self,
        entity: Rc<Entity>,
    ) -> Vec<PointId> {
        let end_points_available: Vec<PointId> = self
            .dijkstra_map
            .get_all_points_with_cost_between(
                Cost(0f32),
                Cost(entity.entity_intern.move_force()),
            )
            .iter()
            .filter(|&x| {
                let x = self.dijkstra_point_id_to_pos.get(x).unwrap();
                self.team_id_to_set_of_position_taken
                    .get(&entity.team)
                    .unwrap()
                    .contains(x)
            })
            .copied()
            .collect();
        end_points_available
    }

    /// Returns every matching ranges where the entity can do something
    /// It serves as a choice provider for entities.
    fn get_actions_for_entity(
        &mut self,
        entity: Rc<Entity>,
        terrain_manager: TerrainManager,
    ) -> Vec<(
        Action,
        SelectorResult,
    )> {
        let mut result: Vec<(
            Action,
            SelectorResult,
        )> = Vec::new();
        let x = entity.entity_intern.ranges_to_actions();

        for (selector, action) in x.into_iter() {
            match selector.select(&self) {
                Some(match_) => result.push((action, match_)),
                None => {}
            }
        }

        // for this_range in entity.entity_intern.get_attack_ranges() {
        //     self.recalculates_dijkstra_map_for_entity_with_force(
        //         entity.clone(),
        //         *this_range as f32,
        //         // this should be a map where every terrain has a weight of one, so the attacks flings no matter the terrain
        //         // OR, we could forbid walls, or other terrain, anyway, needs thinking
        //         terrain_manager.terrain_weight_for_attacks(),
        //     );
        //     let end_points_available = self.dijkstra_map.get_all_points_with_cost_between(
        //         Cost(0f32),
        //         Cost(entity.entity_intern.move_force()),
        //     );

        //     for end_point in end_points_available {
        //         // pour chaque position
        //         let end_point = *self.dijkstra_point_id_to_pos.get(end_point).unwrap();
        //         // si il y a personne, continue
        //         if self.pos_to_occupant.get(&end_point).is_none() {
        //             continue;
        //         }
        //         let occupant = self.pos_to_occupant.get(&end_point).unwrap();
        //         // si il y a un loner, on garde toutes les positions

        //         if let Occupant::Entity(e) = occupant {
        //             // get all set except the one of entity.team
        //             if entity.team.can_fight(&e.team) {
        //                 result.push((
        //                     end_point,
        //                     e.unique_id,
        //                 ))
        //             } else {
        //                 continue;
        //             }
        //         }
        //     }
        // }
        // result
        todo!()
    }

    fn enable_all_dijkstra_points(&mut self) {
        for k in self.pos_to_dijkstra_point_id.values() {
            self.dijkstra_map.enable_point(*k).unwrap();
        }
    }

    /// the entity cannot :
    /// - cross a pos where an enemy entity is
    /// this function should return a vector containing these inaccessible positions
    fn get_uncrossable_points_for_entity(
        &mut self,
        entity: Rc<Entity>,
    ) -> Vec<PointId> {
        let mut uncrossable_points: Vec<PointId> = Vec::new();

        for (team, set) in &self.team_id_to_set_of_position_taken {
            if entity.team.can_fight(team) {
                for pos in set {
                    uncrossable_points.push(*self.pos_to_dijkstra_point_id.get(pos).unwrap())
                }
            }
        }
        uncrossable_points
    }
    /// given a `force`, rebakes the [DijkstraMap] for an entity
    fn recalculates_dijkstra_map_for_entity_with_force(
        &mut self,
        entity: Rc<Entity>,
        force: f32,
        terrain_weights: HashMap<Terrain, f32>,
    ) {
        let position = self.entity_id_to_pos.get(&entity.unique_id).unwrap();
        self.dijkstra_map.recalculate(
            &[*self.pos_to_dijkstra_point_id.get(position).unwrap()],
            None,
            Some(Cost(force)),
            Vec::new(),
            terrains::terrain_weights_to_dijkstra_terrain_weight(&terrain_weights),
            FnvHashSet::default(),
        );
    }
    /// ! if the dji map hasn't been precalculated, this returns empty array
    /// the paths returned doesn't include the point at which the entity is
    fn end_points_ids_to_paths_to_end_points(
        &self,
        end_points_available: &[PointId],
    ) -> Vec<Vec<Pos2D>> {
        let mut paths: Vec<Vec<Pos2D>> = Vec::new();

        for ele in end_points_available {
            let i = self.dijkstra_map.get_shortest_path_from_point(*ele);
            let mut v: Vec<Pos2D> = Vec::new();

            for id in i {
                // turn pointID to pos
                let pos = self.dijkstra_point_id_to_pos.get(&id).unwrap();
                // push pos
                v.push(*pos)
            }
            paths.push(v);
        }
        paths
    }

    /// The idea behind this method is that you provide a description of the objects you want on the map
    /// and it returns a slice of Positions.
    pub fn select(&self) {
        todo!()
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

pub struct UnInitializedMap {
    next_id: i32,
    id_to_terrain: HashMap<i32, Terrain>,
    terrain_to_id: HashMap<Terrain, i32>,
}

impl UnInitializedMap {
    pub(crate) fn new() -> Self {
        UnInitializedMap {
            next_id: 0,
            id_to_terrain: HashMap::default(),
            terrain_to_id: HashMap::default(),
        }
    }

    fn declare_terrain(
        &self,
        _arg: &str,
        _entity_may_cross: TerrainType,
    ) -> i32 {
        todo!()
    }

    fn get(
        &self,
        _id: i32,
    ) -> Terrain {
        todo!()
    }
}

#[cfg(test)]
mod tests;
