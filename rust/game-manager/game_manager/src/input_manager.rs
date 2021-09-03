use super::*;
/// this is how a [GameManager] will communicate what choices are available for currently playing entity
/// it will be cloned and cached by the game_manager so we can use the id to declare the choice
/// spec : an id that will be used to reference the
#[derive(Clone)]
pub struct InputOption {
    /// this id should be unique and will be communicated to the game manager
    /// to ensure this specific [InputOption] will be selected
    pub unique_id: i32,
    /// the [Intent] corresponding
    pub intent: Intent,
}

/// The inputmanager keeps track of the state of the game to always permit to be asked what
/// - inputs must be submitted
/// - what is the context etc
///
/// currently not used, it will be the interface with exterior sources/ godot side?
pub struct InputManager {}
