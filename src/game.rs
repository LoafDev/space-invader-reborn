use bevy::prelude::*;
use crate::GameState;
mod bullet; mod enemy; mod collisions; mod cameras; mod player; mod score; mod paused;

#[derive(SubStates, Clone, Copy, Default, Eq, PartialEq, Debug, Hash)]
#[source(GameState = GameState::InGame)]
pub enum InGameState {
    #[default]
    Playing,
    Paused,
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_sub_state::<InGameState>()
            .enable_state_scoped_entities::<InGameState>()
            .add_plugins((
                cameras::CameraPlugin,
                bullet::BulletPlugin,
                player::PlayerPlugin,
                enemy::EnemyPlugin,
                collisions::CollisionPlugin,
                score::ScorePlugin,
                paused::PausedPlugin
            ))
            .add_systems(Update, take_esc.run_if(in_state(GameState::InGame)));
    }
}

fn take_esc (
    key: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<InGameState>>,
    cur_state: Res<State<InGameState>>
) {
    if key.just_pressed(KeyCode::Escape) {
        next_state.set(
            match cur_state.get() {
                InGameState::Playing => InGameState::Paused,
                InGameState::Paused => InGameState::Playing
            }
        );
    }
}
