use bevy::prelude::*;
use crate::{ constants, game::{collisions, InGameState}, GameState };

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy;

#[derive(Resource)]
struct EnemyManager {
    direction: f32,
    is_shift: bool
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), setup_enemy)
            .add_systems(Update, (move_enemy, shift_enemy).run_if(in_state(InGameState::Playing)));
    }
}

fn setup_enemy (
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.insert_resource( EnemyManager { direction: 1., is_shift: false } );

    let enemy_sprite = asset_server.load("enemy.png");
    let mut position: Vec3;
    
    for x in 0..constants::WIDTH_ENEMY {
        for y in 0..constants::HEIGHT_ENEMY {
            position = Vec3::new(x as f32 * constants::ENEMY_SPACE, y as f32 * constants::ENEMY_SPACE, 0.)
            - Vec3::X * constants::WIDTH_ENEMY as f32 * constants::ENEMY_SPACE * 0.5
            - Vec3::Y * constants::HEIGHT_ENEMY as f32 * constants::ENEMY_SPACE * 0.1;

            commands.spawn((
                Sprite { image: enemy_sprite.clone(), ..Default::default() },
                Transform::from_translation(position).with_scale(Vec3::splat(constants::SCALE_RATIO)),
                Enemy,
                collisions::Collider,
                StateScoped(GameState::InGame)
            ));
        }
    }
}

fn move_enemy (
    mut enemy_q: Query<&mut Transform, With<Enemy>>,
    mut enemy_mq: ResMut<EnemyManager>,
    time: Res<Time>
) {
    for mut transform in enemy_q.iter_mut() {
        transform.translation.x += enemy_mq.direction * constants::ENEMY_SPEED * time.delta_secs();

        if (transform.translation.x >= constants::MAX_WIDTH*0.5 && enemy_mq.direction > 0.) || (transform.translation.x <= constants::MAX_WIDTH*(-0.5) && enemy_mq.direction < 0.) { enemy_mq.is_shift = true; }
    }
}

fn shift_enemy (
    mut enemy: Query<&mut Transform, With<Enemy>>,
    mut manager: ResMut<EnemyManager>
) {
    if manager.is_shift {
        manager.direction *= -1.;
        manager.is_shift = false;

        for mut transform in enemy.iter_mut() { transform.translation.y -= constants::ENEMY_SHIFT; }
    }
}
