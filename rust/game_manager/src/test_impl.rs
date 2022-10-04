use dijkstra_map::grids::Vector2D;

use crate::common_types::{Entity, Position, Selector};
use crate::input_manager::InputManager;
use crate::map::{terrains::TerrainSet, Map};

use super::manager::{GameManager, GameManagerInitialiser};
use super::*;

pub struct MyEntity {
    name: String,
    initiative: f32,
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
        Selector {
            mode: common_types::selector::SelectorMode::Dijkstra,
            filter: todo!(),
        };
        todo!()
    }

    fn can_still_play(&self) -> bool {
        true
    }

    fn get_initiative(&self) -> f32 {
        self.initiative
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
fn basic() {
    let terrains = TerrainSet::new();
    let my_map = Map::new(terrains, (10, 10));
    let mut gm = GameManagerInitialiser::initialise::<MyEntity>(my_map);
    let e1 = MyEntity {
        name: "Robert".into(),
        initiative: 0.0,
    };
    let e2 = MyEntity {
        name: "Jean".into(),
        initiative: 1.0,
    };
    let id1 = gm.register(
        e1,
        common_types::TeamId::Loner,
    );
    let id2 = gm.register(
        e2,
        common_types::TeamId::Loner,
    );
    let pos1: Position = Position(Vector2D::new(0, 0));
    let pos2: Position = Position(Vector2D::new(2, 2));
    gm.try_place(id1, pos1).unwrap();
    gm.try_place(id2, pos2).unwrap();

    let im = InputManager::new(gm);

    let pe = im.get_playable_entities();
    dbg!(&pe);
    // assert not empty
    let opt = im.get_options_for_entity(0, pe);
    // assert not empty
    // im.play(0, opt);
    // todo!();
}
