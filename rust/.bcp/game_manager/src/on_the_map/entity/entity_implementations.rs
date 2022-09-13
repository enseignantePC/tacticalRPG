use crate::Attack;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ClassicEntity {
    move_force: f32,
    damage_reduction: f64,
    initiative: f64,
    played_this_turn: bool,
    id: EntityId,
    strength: f64,
    crit_chance: f32,
}

impl EntityIntern for ClassicEntity {
    fn terrain_weights(&self) -> HashMap<Terrain, f32> {
        let mut x = HashMap::<Terrain, f32>::new();
        let t = Terrain {
            unique_id: 0,
            name: "Grass".into(),
            _type: crate::map::terrains::TerrainType::EntityMayCross,
        };
        x.insert(t, 1.0);
        x
    }

    fn move_force(&self) -> f32 {
        self.move_force
    }

    fn damage_reduction_factor(&self) -> f64 {
        self.damage_reduction
    }

    fn initiative(&self) -> f64 {
        self.initiative
    }

    fn can_play(&self) -> bool {
        !self.played_this_turn
    }

    fn selector_map(&self) -> HashMap<Selector, Action> {
        let mut hm = HashMap::new();
        hm.insert(
            Selector {
                excluded_entity: todo!(),
                mask: todo!(),
                pattern: todo!(),
            },
            Action::Attack(Attack {
                _type: crate::AttackType::Base,
                strength: self.strength,
                crit_chance: self.crit_chance,
                // target: todo!(),
            }),
        );
        hm
        // todo!()
    }

    fn action_possible_to_intent(
        &self,
        action: Action,
        context: SelectorResult,
    ) -> Intent {
        todo!()
    }
}
