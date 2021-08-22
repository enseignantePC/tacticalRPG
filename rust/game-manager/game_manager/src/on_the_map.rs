use super::TeamID;
/// "live" thing interacting in the world
pub struct Entity {
    team: TeamID,
}
/// interactive object present on the map (interruptor, usable etc)
pub struct Object {}
/// movable or destructable (or interactable?) obstacle on the map
pub struct Obstacle {}
