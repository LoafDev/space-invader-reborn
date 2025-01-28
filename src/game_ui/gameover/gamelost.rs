use bevy::prelude::*;
use crate::GameState;

#[derive(Component)]
struct WhileInGameLost;

pub struct GameLostPlugin;
impl Plugin for GameLostPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::GameLost), setup_gamelost)
            .add_systems(OnExit(GameState::GameLost), crate::despawn_screen::<WhileInGameLost>);
    }
}

fn setup_gamelost() { println!("Welcome to the after life!"); }
