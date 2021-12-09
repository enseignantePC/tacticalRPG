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
        dict.insert(
            "unique_id",
            self.unique_id,
        );
        gdnative::core_types::Variant::from_dictionary(&dict.into_shared())
    }
}

pub trait EntityIntern: Debug {
    /// determines how easily the entity travels different terrain types
    /// TODO say more about dijkstra, where are we about reworking terrains?
    fn terrain_weights(&self) -> HashMap<Terrain, f32>;
    /// determines how far the entity will be able to move
    fn get_move_force(&self) -> f32;
    /// damage reduction when fighting, should depend on SOMETHING
    fn damage_reduction_factor(&self) -> f64;
    /// how likely to play before other entities.
    /// TODO IS THIS DETERMINIST???
    fn initiative(&self) -> f64;
    /// whether the entity can play other moves in the turn or is it `exhausted`.
    /// ! WARNING if the entity returns can_play == true, it should return non empty input options
    /// ! when queried! Otherwise the GameManager will have trouble knowing when the turn is over.
    /// ! In the end the GameManager should become smart enough to `eliminate` a player when
    /// ! it says it can play but return no options.
    ///
    /// ! This is subtle because the player answer if they can play according to their state,
    /// ! their actual options is not their scope.
    /// ! So how should the game manager decide when the turn is over?
    /// ! It could check before returning from get_playable_entities that the entities return
    /// ! have at least one option. Find the first team that does. If no such team exist the
    /// ! turn is terminated.
    /// !
    /// ! But what to do to avoid infinite loops in the case where all Entities could play
    /// ! but they are blocked? ...
    fn can_play(&self) -> bool;
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
    pub fn test_entity(
        team_id: Option<i64>,
        id: Option<i64>,
    ) -> Self {
        #[derive(Debug)]
        struct Intern {}
        impl EntityIntern for Intern {
            fn terrain_weights(&self) -> HashMap<Terrain, f32> {
                panic!()
            }

            fn get_move_force(&self) -> f32 {
                panic!()
            }

            fn damage_reduction_factor(&self) -> f64 {
                panic!()
            }

            fn get_attack_ranges(&self) -> &[i32] {
                panic!()
            }

            fn initiative(&self) -> f64 {
                todo!()
            }

            fn can_play(&self) -> bool {
                todo!()
            }
        }
        Entity {
            team: TeamId::Team(team_id.unwrap_or(0)),
            unique_id: EntityId(id.unwrap_or(0)),
            entity_intern: Box::new(Intern {}),
        }
    }
}
