// TODO branch note: this should be the game_manager_wrapper_of a entity_intern that is a trait
/// An entity is the type of thing on the map that requires input to decide what to do.
/// You should think of entities as the only "alive" thing on the map
use super::*;

/// As entities are the most important thing on the map
/// they should be the most flexible thing possible
///
/// It makes more and more sense that it should be a trait
/// and maybe the entity should be handled entirerly by the external source,
/// possibly in the form of a Godot Node, capable of calling some gdscript code to satisfy the trait
/// (which would probably make it unsafe)?
///
/// As i am too much of a newbie at rust, this frigthens me however
#[derive(Debug)]
pub struct Entity {
    pub team: TeamId,
    pub unique_id: EntityId,
    pub entity_intern: Box<dyn EntityIntern>,
}

impl ToVariant for Entity {
    fn to_variant(&self) -> gdnative::core_types::Variant {
        let dict = gdnative::core_types::Dictionary::new();
        gdnative::core_types::Variant::from_dictionary(&dict)
    }
}
impl Entity {
    pub fn can_attack(&self, other_entity: &Entity) -> bool {
        if self.team == TeamID::Loner || other_entity.team == TeamID::Loner {
            true
        } else {
            self.team != other_entity.team
        }
    }
}

pub trait EntityIntern: Debug {
    fn terrain_weights(&self) -> HashMap<TerrainType, f32>;
    /// determines how far the entity will be able to move
    fn get_move_force(&self) -> f32;

    /// damage reduction when fighting, should depend on SOMETHING
    fn damage_reduction_factor(&self) -> f64;

    /// At what distance(s) the entity can strike
    /// currently broken but should depend on
    /// - the weapon
    /// - the entity
    /// - ? a plethora of other stuff, should the logic be handled by the external source?
    fn get_attack_ranges(&self) -> &[i32];
}