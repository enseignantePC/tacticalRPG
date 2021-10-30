use super::*;

/// TODO should accept more context :
/// - from what terrains to what terrains does the entity attack
/// - is it raining or whatever
/// what kind of curse (tags?) are on the map for both people
pub fn solve(
    attack: Attack,
    entity_assailant: &Entity,
    entity_attacked: &Entity,
) -> ResolvedAttack {
    let damage: f64 = attack.strength * entity_attacked.entity_intern.damage_reduction_factor();
    ResolvedAttack {
        damage_dealt: damage,
        critical_hit: false,
    }
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug, ToVariant)]
pub struct ResolvedAttack {
    damage_dealt: f64,
    critical_hit: bool,
}
