use bevy::prelude::*;
use crate::{constants, game::InGameState};

pub struct BulletPlugin;

#[derive(Resource)]
pub struct BulletImage(pub Handle<Image>);

#[derive(Component)]
pub struct EnemyMarker;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Velocity(pub f32);

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_bullet)
            .add_systems(Update, apply_velocity.run_if(in_state(InGameState::Playing)));
    }
}

fn setup_bullet (
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.insert_resource(BulletImage(asset_server.load("bullet.png")));
}

fn apply_velocity (
    mut commands: Commands,
    mut query: Query<(Entity, &Velocity, &mut Transform), With<Velocity>>,
    time: Res<Time>
) {
    for (entity, velocity, mut transform) in query.iter_mut() {
        let movement = velocity.0 * time.delta_secs();
        let movement_direction = transform.rotation * Vec3::Y;
        transform.translation += movement_direction * movement;

        if transform.translation.x.abs() >= constants::MAX_WIDTH*0.5 || transform.translation.y.abs() >= constants::MAX_HEIGHT*0.5 { commands.entity(entity).despawn(); }
    }
}
