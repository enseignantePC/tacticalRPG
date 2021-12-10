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
pub struct Selector {
    /// A list of entity that cannot be selected by the range,
    /// useful for [TeamId::Loner] so they cannot target themselves...
    excluded_entity: Vec<EntityId>,
    /// A descriptor of which team to match
    team_mask: TeamMask,
    /// Each of this range will be checked for the position
    /// and the matches returned, possibly with duplicates
    /// that means the target counts multiple times.
    pattern: Pattern,
}

impl Selector {
    pub fn select(
        self,
        map: &Map,
    ) -> Option<SelectorResult> {
        let Selector {
            excluded_entity,
            team_mask,
            pattern,
        }: Selector = self;
        let (result, _) = pattern.select(0);
        todo!()
    }
}

/// The search algorithm is dependant on kind, The search algorithm is applied
/// on positions yielded by inner patterns if any, otherwise on a simple Pos
pub struct Pattern {
    kind: PatternKind,
    relative: Either,
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
enum Either {
    Pos(Pos2D),
    Patterns(Vec<Pattern>),
}

pub enum PatternKind {
    // yield the points that are between min_dist_from_point
    // and min_dist_from_point + len
    // on the [DijkstraMap]
    DijkstraCross(i32, u32, TerrainMap),
    /// the shape, described as a set of position
    /// everything in the shape, around the current point will be selected.
    Shape(Vec<Pos2D>),
    /// Will match if in direction (at max_distance) (through max matches)
    /// note that this doesn't go through portals.
    /// TODO check in direction (Pos2D) and map is linked in dijkstra_map (DijkstraMapPosId)
    /// TODO a ForcedDirection that goes through
    Direction,
    /// Custom Iterator that gives Pos that will be checked in that order.
    /// TODO : most complicated example : A ForcedDirection that go
    /// TODO : through portals (at each out points)
    /// TODO : and change direction then.
    Closure,
}

impl PatternKind {
    fn select(
        &self,
        pos: &Pos2D,
    ) -> Vec<PatternResult> {
        match self {
            PatternKind::DijkstraCross(_, _, _) => todo!(),
            PatternKind::Shape(_) => todo!(),
            PatternKind::Direction => todo!(),
            PatternKind::Closure => todo!(),
        }
    }
}

impl Pattern {
    fn select(
        &self,
        depth_level: i32,
    ) -> (
        Vec<PatternResult>,
        i32,
    ) {
        match &self.relative {
            Either::Pos(pos) => (
                self.kind.select(pos),
                depth_level,
            ),
            // !TODO! This is probably logically wrong
            Either::Patterns(x) => (
                x.iter()
                    .map(|y| y.select(depth_level + 1))
                    .map(|(p, depth)| p)
                    .flatten()
                    .collect(),
                depth_level + 1,
            ),
        }
    }
}
/// When a selector matched, it remembers what entities it matched
/// where they where on the map and with what ranged it matched them.
/// The first matches accessible.
///
/// The impl of this should ensure that if you have a Match struct,
/// at least one target is inside. That's why [Selector::select] returns
/// an option.
pub struct SelectorResult;

/// When a Pattern matches, it remembers  it's depth level and Entity result
/// it got along the way.
/// - Every position matching the query
/// - Every EntityId matching the query
struct PatternResult {
    position_matches: Vec<Pos2D>,
    entity_matches: Vec<(EntityId, Pos2D)>,
}
