use bevy::prelude::*;
mod gamewin; mod gamelost;

pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((gamewin::GameWinPlugin, gamelost::GameLostPlugin));
    }
}
