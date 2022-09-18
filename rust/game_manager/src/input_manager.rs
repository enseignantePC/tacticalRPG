use crate::{
    common_types::{Entity, Intent, WorldChange},
    manager::{EntityIntern, GameManager},
    map::Map,
};

pub struct PlayableEntities;
pub struct SelectableIntents;
pub struct InputManager<T: Entity> {
    game_manager: GameManager<T>,
}

impl<T: Entity> InputManager<T> {
    pub fn new(game_manager: GameManager<T>) -> Self {
        InputManager { game_manager }
    }
    pub fn get_playable_entities(&self) -> PlayableEntities {
        todo!()
    }

    pub fn get_options_for_entity(
        &self,
        choice: i32,
        entities: PlayableEntities,
    ) -> SelectableIntents {
        let entity = entities.choice::<T>(choice).unwrap();
        // let res = entity.entity.get_play_options();
        let mut res = vec![];
        for (selector, action) in entity.entity.get_play_options() {
            let this = self.game_manager.map_select(selector);
            if this.is_not_empty() {
                res.push(this.to_intent(action));
            }
        }
        SelectableIntents::from(res)
    }

    pub fn play(
        &mut self,
        choice: i32,
        selectable_intents: SelectableIntents,
    ) -> Vec<WorldChange<T::EntityChange>> {
        let i: Intent = selectable_intents.select(choice);
        self.game_manager.submit_intent(i)
    }
}

impl PlayableEntities {
    fn choice<T: Entity>(
        self,
        choice: i32,
    ) -> Result<EntityIntern<T>, ()> {
        todo!();
    }
}

impl SelectableIntents {
    pub fn from(intents: Vec<Intent>) -> Self {
        todo!()
    }

    pub fn select(
        self,
        choice: i32,
    ) -> Intent {
        todo!()
    }
}
