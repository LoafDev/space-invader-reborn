use bevy::prelude::*;
use crate::GameState;
mod menu; mod gameover;

#[derive(Component)]
pub enum MenuButton {
    Play,
    Credit,
    Exit,
    Back
}

pub struct GameUiPlugin;
impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((menu::MenuPlugin, gameover::GameOverPlugin))
            .add_systems(Update, press_button.run_if(not(in_state(GameState::InGame))));
    }
}

fn press_button (
    interact_query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut game_exit: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<menu::MenuState>>,
    mut game_state: ResMut<NextState<GameState>>
) {
    for (interaction, menu_button) in &interact_query {
        if *interaction == Interaction::Pressed {
            match menu_button {
                MenuButton::Play => { game_state.set(GameState::InGame); }
                MenuButton::Credit => { menu_state.set(menu::MenuState::InCredit); }
                MenuButton::Exit => { game_exit.send(AppExit::Success); }
                MenuButton::Back => { game_state.set(GameState::Menu); menu_state.set(menu::MenuState::InMain); }
            }
        }
    }
}
