- [] Add a way to define spell easily
- [] Add tests for WorldManager
- [] Add tests for map state after World::apply_change_to_world
- [] Add tests for Watcher
- [] Add integration tests for GameManager
- [] Add a few custom spell and test them
- [] Design a better system for attack resolution
- [] Design a system for Ranges (applied to attacks and spells and objects) 
Range {
    Shape {
        // map.cost_in_between, with a special dict of Terrain -> Cost = 1 or infinity
        // each of the cases in there can be selected
        Distance(min u64, max i64,Some(through walls)),
        // Pos(0,0) is the case selected in the distance step, all other case will also be checked for targets
        shape(Some(Vec<Pos2D>) = (0,0))
    }
    Direction {
        a direction checked (each case through max distance, through max target)
        Pos2D //the direction
        max_distance
        max_target
    }
}
- [] Design a system for selecting targets on the map that takes
    - a vec of range
    - a vec of team_filter (all, same_team,other_team,loner,Id(x))