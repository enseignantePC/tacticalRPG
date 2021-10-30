use std::rc::Rc;

use super::*;

/// TODO should accept more context :
/// - from what terrains to what terrains does the entity attack
/// - is it raining or whatever
/// what kind of curse (tags?) are on the map for both people
pub fn solve(
    entity_id_to_entity: &HashMap<EntityId, Rc<Entity>>,
    attack: Attack,
    entity_attacking: Rc<Entity>,
) -> ResolvedAttack {
    let entity_attacked = entity_id_to_entity.get(&attack.target).expect(
        "couldn't find entity corresponding to id {} while the entity is a target of an attack",
    );
    let damage: f64 = attack.strength * entity_attacked.entity_intern.damage_reduction_factor();
    ResolvedAttack {
        from: entity_attacking.unique_id,
        to: entity_attacked.unique_id,
        damage_dealt: damage,
        critical_hit: false,
    }
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug, ToVariant)]
pub struct ResolvedAttack {
    from: EntityId,
    to: EntityId,
    damage_dealt: f64,
    critical_hit: bool,
}
