use bevy::prelude::*;
mod bullet; mod enemy; mod collisions; mod cameras; mod player; mod score;

#[derive(Component)]
pub struct WhileInGame;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                cameras::CameraPlugin,
                bullet::BulletPlugin,
                player::PlayerPlugin,
                enemy::EnemyPlugin,
                collisions::CollisionPlugin,
                score::ScorePlugin
            ))
            .add_systems(OnExit(crate::GameState::InGame), crate::despawn_screen::<WhileInGame>);
    }
}
