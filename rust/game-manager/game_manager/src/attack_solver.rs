use super::*;

/// TODO should accept more context :
/// -from what terrains to what terrains does the entity attack
/// - is it raining or whatever
/// what kind of curse (tags?) are on the map for both people
pub fn solve(attack: Attack, entity_assailant: &Entity, entity_attacked: &Entity) -> AttackResult {
    let damage: f64 = attack.strength * entity_attacked.entity_intern.damage_reduction_factor();
    AttackResult {
        damage_dealed: damage,
        critical_hit: false,
    }
}

// #[derive(PartialEq, PartialOrd, Clone, Copy, Debug, ToVariant)]
pub struct AttackResult {
    damage_dealed: f64,
    critical_hit: bool,
}
