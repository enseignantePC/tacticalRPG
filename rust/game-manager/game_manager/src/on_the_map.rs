/// everything that is on the map
///
use super::TeamID;

/// An entity is the only thing on the map that requires input to decide what to do
pub struct Entity {
    team: TeamID,
}

/// interactive object present on the map (interruptor, usable etc)
pub struct Object {}
/// movable or destructable (or interactable?) obstacle on the map
pub struct Obstacle {}

impl Entity {
    /// creates a entity for test purposes
    pub fn example_entity() -> Entity {
        Entity {
            team: TeamID::Loner,
        }
    }
    // TODO : properly implement this
    #[deprecated = "prout"]
    pub fn damage_reduction_factor(&self) -> f64 {
        1f64
    }
}
