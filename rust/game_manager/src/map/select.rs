use super::*;

/// You provide a Query to the select function
/// to describe what can of search you want it to perform.
pub struct Query {}

pub struct Selector {
    /// the current Pos being searched for the query.
    current_position: Pos2D,
    /// A descriptor of which things to match
    team_mask: TeamMask,
    /// Each of this range will be checked for the position
    /// and the matches returned, possible with duplicates.
    ranges: Vec<Range>,
}
/// all ranges search are relative to a position in the [Selector]
pub enum Range {
    /// the shape, described as a set of position
    /// everything in the shape will be selected.
    /// it will work even if the Pos2D aren't connected in the map
    Shape,
    /// Will match if in direction (at max_distance) (through max matches)
    /// note that this doesn't go through portals.
    /// TODO check in direction (Pos2D) and map is linked in dijkstra_map (DijkstraMapPosId)
    /// TODO a ForcedDirection that goes through
    Direction,
    /// Custom Iterator that gives Pos that will be checked in that order.
    /// TODO : most complicated example : A ForcedDirection that go
    /// TODO : through portals (at each out or one chosen at random)
    /// TODO : and change direction then.
    Closure,
}
