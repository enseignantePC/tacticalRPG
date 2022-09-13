//! The map::select Module
//!
//! It provides the [Selector] type which can be used to query
//! what entities match certain criteria via a powerful select system.

use super::*;

/// Module containing common selectors for convenience.
pub mod common_selectors;

/// The selector is the top level unit of a query, every result
/// yielded by a selector are linked. If you want Independent result
/// use multiple selectors.
///
/// Selector is built upon the [Pattern] type which expresses what
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Selector {
    /// A list of entity that cannot be selected by the range,
    /// useful for [TeamId::Loner] so they cannot target themselves...
    excluded_entity: Vec<EntityId>,
    /// A descriptor of which team to match
    mask: Mask,
    /// Each of this range will be checked for the position
    /// and the matches returned, possibly with duplicates
    /// that means the target counts multiple times.
    pattern: Pattern,
}

impl Selector {
    pub fn select(
        self,
        map: &mut Map,
    ) -> Option<SelectorResult> {
        let Selector {
            excluded_entity,
            mask,
            pattern,
        } = self;
        let _result = pattern.select(map, &mask);
        todo!()
    }
}

/// The search algorithm is dependant on kind, The search algorithm is applied
/// on positions yielded by inner patterns if any, otherwise on a simple Pos
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Pattern {
    kind: PatternKind,
    relative: PromisedPositions,
}

/// if the variant is Pos, the pattern will yield
/// every pos relative to this position.
///
/// if the variant is Patterns, it will yield position relative
/// to each position yielded by the inner patterns.
///
/// When a pattern match a target, it will record at the
/// appropriate depth level of the match AND keep using position
/// for search for upper level patterns if any.
#[derive(Debug, Clone, PartialEq, Hash)]
enum PromisedPositions {
    Pos(Pos2D),
    Patterns(Vec<Pattern>),
}

/// Describing what occupant should not be
/// selected (and their position should be disabled)
/// while performing a search
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mask {
    TeamMask(TeamMask),
    Exclude(Vec<EntityId>),
    Include(Vec<EntityId>),
}

impl Mask {
    /// disable all the points that shouldn't be included in the search,
    /// this can happen because team A shouldn't be able to walk on points
    /// occupied by an enemy team B for instance.
    pub fn disable_points(
        &self,
        _map: &mut Map,
    ) {
        todo!()
    }
    /// returns wether an occupant should be selected by the mask
    pub fn accept_occupant(
        &self,
        occupant: &Occupant,
    ) -> bool {
        match occupant {
            Occupant::Entity(id) => match &self {
                Mask::TeamMask(_tm) => match _tm {
                    TeamMask::AllExcept(_tms) => {
                        for k in _tms {
                            k.filter(todo!());
                        }
                        todo!();
                    }
                    TeamMask::ThisTeam(_t) => todo!(),
                    TeamMask::NotThisTeam(_t) => todo!(),
                },
                Mask::Exclude(vec_id) => {
                    if let Occupant::Entity(id) = occupant {
                        !vec_id.contains(id)
                    } else {
                        true
                    }
                }
                Mask::Include(vec_id) => {
                    if let Occupant::Entity(id) = occupant {
                        vec_id.contains(id)
                    } else {
                        false
                    }
                }
            },
            Occupant::Obstacle(_) => todo!(),
            Occupant::Object(_) => todo!(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum PatternKind {
    // yield the points that are between min_dist_from_point
    // and min_dist_from_point + len
    // on the [DijkstraMap]
    Dijkstra(f32, f32, TerrainMap),
    /// the shape, described as a set of position
    /// everything in the shape, around the current point will be selected.
    Shape(Vec<Pos2D>),
    /// Will match if in direction (at max_distance) (through max matches)
    /// note that this doesn't go through portals.
    /// TODO check in direction (Pos2D) and map is linked in dijkstra_map (DijkstraMapPosId)
    /// TODO a ForcedDirection that goes through ?
    Direction,
    /// Custom Iterator that gives Pos that will be checked in that order.
    /// TODO : most complicated example : A ForcedDirection that go
    /// TODO : through portals (at each out points)
    /// TODO : and change direction then.
    Closure,
}

impl PatternKind {
    /// TODO : this actually does the work only for entity! what happens if you match an occupant,
    /// TODO : it should deal with that too!
    fn select(
        &self,
        position: &Pos2D,
        map: &mut Map,
        mask: &Mask,
    ) -> PatternResult {
        match self {
            PatternKind::Dijkstra(min_dist, dist_len, terrain_map) => {
                let mut to_return = PatternResult {
                    position_matches: vec![],
                    entity_matches: vec![],
                };
                map.recalculates_dijkstra_map_at_pos_with_force(
                    position,
                    min_dist + dist_len,
                    terrain_map,
                );
                // disable every forbidden points so they cannot be crossed nor selected
                mask.disable_points(map);

                for point in map.dijkstra_map.get_all_points_with_cost_between(
                    Cost(*min_dist),
                    Cost(min_dist + dist_len),
                ) {
                    let pos_match = map.dijkstra_point_id_to_pos.get(point).expect(
                        "Could get an id for a non existent position in the \
                        map from a selector, this should not be possible for a map",
                    );
                    to_return.position_matches.push(*pos_match);
                    let occupant = map.pos_to_occupant.get(pos_match);
                    if let Some(occupant) = occupant {
                        if mask.accept_occupant(occupant) {
                            to_return.entity_matches.push((
                                (*occupant).clone(),
                                *pos_match,
                            ))
                        }
                    }
                }
                to_return
            }
            PatternKind::Shape(shape_positions) => {
                let mut to_return = PatternResult {
                    position_matches: vec![],
                    entity_matches: vec![],
                };
                for each_pos in shape_positions {
                    let next_pos = each_pos + position;
                    let occupant = map.pos_to_occupant.get(&next_pos);
                    if let Some(occupant) = occupant {
                        if mask.accept_occupant(occupant) {
                            to_return.entity_matches.push((
                                (occupant).clone(),
                                next_pos,
                            ))
                        }
                    }
                    if occupant.is_some() && mask.accept_occupant(occupant.unwrap()) {}
                }
                to_return
            }
            PatternKind::Direction => {
                // get the position via  Dijkstra and then check if they are always
                // in the same direction?
                todo!()
            }
            PatternKind::Closure => todo!(),
        }
    }
}

/// returns a vec of patterns result, the last result
/// in the vector are the deepest one.
impl Pattern {
    fn select(
        &self,
        _map: &mut Map,
        _mask: &Mask,
    ) -> HashMap<i32, Vec<PatternResult>> {
        todo!()
        // match &self.relative {
        //     Either::Pos(pos) => {
        //         let mut result = HashMap::new();
        //         result.insert(
        //             0,
        //             vec![self.kind.select(
        //                 &pos.clone(),
        //                 map,
        //                 mask,
        //             )],
        //         );
        //         result
        //     }
        //     // !TODO! This is probably logically wrong
        //     Either::Patterns(inner_patterns) => {
        //         // you find the deepest patterns with relative = pos and compute them
        //         // then you get up one level and compute what was relative
        //         // again till ur top
        //         for pattern in inner_patterns.iter() {
        //             let index = 0;
        //             // if not relative, add it to the stack as is,
        //             // else, compute deeper
        //             let mut stack: HashMap<i32, Vec<PatternResult>> = HashMap::default();
        //             match &pattern.relative {
        //                 Either::Pos(_) => {
        //                     // add a vec at index 0 if is none, add results to the vec
        //                     let mut select = pattern.select(map, mask);
        //                     let result = select.get(&0).unwrap();
        //                     match &mut stack.get(&index) {
        //                         Some(x) => x.append(&mut result.clone()),
        //                         None => {
        //                             stack.insert(
        //                                 index,
        //                                 result.clone(),
        //                             );
        //                         }
        //                     }
        //                 }
        //                 Either::Patterns(_inner) => todo!(),
        //             }
        //         }
        //         todo!()
        //     }
        // }
    }
}
/// When a selector matched, it remembers what entities it matched
/// where they where on the map and with what ranged it matched them.
/// The first matches accessible.
///
/// The impl of this should ensure that if you have a Match struct,
/// at least one target is inside. That's why [Selector::select] returns
/// an option.
#[derive(Debug, Clone)]
pub struct SelectorResult;

/// When a Pattern matches, it remembers  it's depth level and Entity result
/// it got along the way.
/// - Every position matching the query
/// - Every EntityId matching the query
///
/// TODO : this actually does the work only for entity! what happens if you match an occupant,
/// TODO : it should deal with that too!
#[derive(Clone)]
struct PatternResult {
    position_matches: Vec<Pos2D>,
    entity_matches: Vec<(Occupant, Pos2D)>,
}
