//! An entity is the type of thing on the map that requires input to decide what to do.
//! You should think of entities as the only "alive" thing on the map
// TODO branch note: this should be the game_manager_wrapper_of a entity_intern that is a trait

use gdnative::core_types::ToVariant;

use super::*;

/// As entities are the most important thing on the map
/// they should be the most flexible thing possible
///
/// It makes more and more sense that it should be a trait.
/// possibly in the form of a Godot Node, capable of calling some
/// gdscript code to satisfy the trait
/// (which would probably make it unsafe)?
///
/// or maybe the entity should be handled entirely by the external source
/// As i am too much of a newbie at rust, this is a source of worry.
#[derive(Debug)]
pub struct Entity {
    pub team: TeamId,
    pub unique_id: EntityId,
    pub entity_intern: Box<dyn EntityIntern>,
}

impl ToVariant for Entity {
    fn to_variant(&self) -> gdnative::core_types::Variant {
        let dict = gdnative::core_types::Dictionary::new();
        dict.insert("team", self.team);
        dict.insert("unique_id", self.unique_id);
        gdnative::core_types::Variant::from_dictionary(&dict.into_shared())
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
    // TODO : make this a more complex Range struct that can deal with some different logic
    fn get_attack_ranges(&self) -> &[i32];
}

#[cfg(test)]
impl Entity {
    pub fn test_entity(entity: Option<i64>, id: Option<i64>) -> Self {
        #[derive(Debug)]
        struct Intern {}
        impl EntityIntern for Intern {
            fn terrain_weights(&self) -> HashMap<TerrainType, f32> {
                todo!()
            }

            fn get_move_force(&self) -> f32 {
                todo!()
            }

            fn damage_reduction_factor(&self) -> f64 {
                todo!()
            }

            fn get_attack_ranges(&self) -> &[i32] {
                todo!()
            }
        }
        Entity {
            team: TeamId::Team(entity.unwrap_or(0)),
            unique_id: EntityId(id.unwrap_or(0)),
            entity_intern: Box::new(Intern {}),
        }
    }
}
