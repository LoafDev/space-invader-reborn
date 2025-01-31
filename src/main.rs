use bevy::prelude::*;
mod game; mod constants; mod game_ui;

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Menu,
    InGame,
    GameLost,
    GameWin
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins
        .set(WindowPlugin{
            primary_window: Some(Window {
                title: String::from("Nah space invader'd win"),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                resolution: Vec2::new(constants::MAX_HEIGHT, constants::MAX_WIDTH).into(),
                ..Default::default()
            }),
            ..Default::default()
        })
        .set(ImagePlugin::default_nearest()),
    )
    .init_state::<GameState>()
    .enable_state_scoped_entities::<GameState>()
    .add_plugins((game::GamePlugin, game_ui::GameUiPlugin))
    .run();
}
