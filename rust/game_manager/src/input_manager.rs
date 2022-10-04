use crate::{
    common_types::{Entity, EntityId, Intent, WorldChange},
    manager::{EntityIntern, GameManager},
    map::Map,
};

#[derive(Debug)]
pub struct PlayableEntities(Vec<EntityId>);
#[derive(Debug)]
pub struct SelectableIntents;
#[derive(Debug)]
pub struct InputManager<T: Entity> {
    game_manager: GameManager<T>,
}

impl<T: Entity> InputManager<T> {
    pub fn new(game_manager: GameManager<T>) -> Self {
        InputManager { game_manager }
    }
    pub fn get_playable_entities(&self) -> PlayableEntities {
        PlayableEntities::from(self.game_manager.get_playable_entities())
    }

    pub fn get_options_for_entity(
        &self,
        choice: usize,
        entities: PlayableEntities,
    ) -> Result<SelectableIntents, ()> {
        let entity = entities.choice(choice)?;
        // let res = entity.entity.get_play_options();
        let mut res = vec![];
        for (selector, action) in self.game_manager.get_play_options_for(entity) {
            let this = self.game_manager.map_select(entity,selector);
            if this.is_not_empty() {
                res.push(this.to_intent(action));
            }
        }
        Ok(SelectableIntents::from(res))
    }

    pub fn play(
        &mut self,
        choice: i32,
        selectable_intents: SelectableIntents,
    ) -> Vec<WorldChange<T>> {
        let i: Intent = selectable_intents.select(choice);
        self.game_manager.submit_intent(i)
    }
}

impl PlayableEntities {
    fn from(x: Vec<EntityId>) -> Self {
        PlayableEntities(x)
    }

    fn choice(
        self,
        index: usize,
    ) -> Result<EntityId, ()> {
        self.0.get(index).and_then(|x| Some(*x)).ok_or(())
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
