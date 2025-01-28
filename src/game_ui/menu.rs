use bevy::prelude::*;
use crate::{GameState, game_ui::MenuButton};
mod credit; mod main_menu;

#[derive(SubStates, Clone, Copy, Default, Eq, PartialEq, Debug, Hash)]
#[source(GameState = GameState::Menu)]
pub enum MenuState {
    #[default]
    InMain,
    InCredit,
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_sub_state::<MenuState>()
            .add_plugins((main_menu::MainMenuPlugin, credit::CreditPlugin));
    }
}
