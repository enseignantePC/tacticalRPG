use crate::common_types::Entity;
use crate::input_manager::InputManager;
use crate::map::{Map, TerrainSet};

use super::manager::{GameManager, GameManagerInitialiser};
use super::*;

pub struct MyEntity {
    name: String,
}

#[derive(Clone)]
pub struct MyMessage;
#[derive(Clone)]

pub struct MyEntityChange;

impl Entity for MyEntity {
    type Message = MyMessage;

    type EntityChange = MyEntityChange;

    fn get_play_options(
        &self
    ) -> Vec<(
        common_types::Selector,
        common_types::Action,
    )> {
        todo!()
    }

    fn can_still_play(&self) -> bool {
        todo!()
    }

    fn get_initiative(&self) -> f32 {
        todo!()
    }

    fn turn_finished(&mut self) {
        todo!()
    }

    fn get_message(
        &mut self,
        msg: Self::Message,
    ) -> common_types::WorldChange<Self::EntityChange> {
        todo!()
    }
}

#[test]
fn feature() {
    let terrains = TerrainSet::new();
    let my_map = Map::new(terrains, (10, 10));
    let mut gm = GameManagerInitialiser::initialise::<MyEntity>(my_map);
    let e1 = MyEntity {
        name: "Robert".into(),
    };
    let e2 = MyEntity {
        name: "Jean".into(),
    };
    let id1 = gm.register(e1);
    let id2 = gm.register(e2);

    let mut im = InputManager::new(gm);

    let pe = im.get_playable_entities();
    let opt = im.get_options_for_entity(0, pe);
    im.play(0, opt);
    todo!();
}
