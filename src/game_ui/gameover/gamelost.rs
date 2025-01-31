use bevy::prelude::*;
use crate::GameState;

pub struct GameLostPlugin;
impl Plugin for GameLostPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::GameLost), setup_gamelost);
    }
}

fn setup_gamelost() { println!("Welcome to the after life!"); }
