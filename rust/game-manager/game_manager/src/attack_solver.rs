use super::*;
pub fn solve(attack: Attack, entity_assailant: &Entity, entity_attacked: &Entity) -> AttackResult {
    let damage: f64 = attack.strength * entity_attacked.entity_intern.damage_reduction_factor();
    AttackResult {
        damage_dealed: damage,
    }
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct AttackResult {
    damage_dealed: f64,
}
