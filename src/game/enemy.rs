use std::time::Duration;
use bevy::{prelude::*, time::common_conditions::on_timer};
use crate::{ constants, game::{bullet, collisions, InGameState}, GameState };

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy;

#[derive(Resource)]
struct EnemyManager {
    direction: f32,
    is_shift: bool,
    enemy_id: Vec<Vec3>,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource( EnemyManager { direction: 1., is_shift: false, enemy_id: Vec::with_capacity(constants::WIDTH_ENEMY) } )
            .add_systems(OnEnter(GameState::InGame), setup_enemy)
            .add_systems(Update, (
                move_enemy,
                shift_enemy,
                enemy_shoot.run_if(on_timer(Duration::from_secs(constants::ENEMY_SHOOT_COOLDOWN)))
            ).run_if(in_state(InGameState::Playing)));
    }
}

fn setup_enemy (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut enemy_manager: ResMut<EnemyManager>
) {
    enemy_manager.direction = 1.;
    enemy_manager.is_shift = false;
    enemy_manager.enemy_id = Vec::with_capacity(constants::WIDTH_ENEMY);

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
            if y == 0 { enemy_manager.enemy_id.push(position); }
        }
    }
}

fn move_enemy (
    mut enemy_q: Query<&mut Transform, With<Enemy>>,
    mut enemy_mq: ResMut<EnemyManager>,
    time: Res<Time>
) {
    let dir = enemy_mq.direction;

    for mut transform in enemy_q.iter_mut() {
        transform.translation.x += dir * constants::ENEMY_SPEED * time.delta_secs();

        if (transform.translation.x >= constants::MAX_WIDTH*0.5 && enemy_mq.direction > 0.) || (transform.translation.x <= constants::MAX_WIDTH*(-0.5) && enemy_mq.direction < 0.) { enemy_mq.is_shift = true; }
    }
    for pos in enemy_mq.enemy_id.iter_mut() { pos.x += dir * constants::ENEMY_SPEED * time.delta_secs(); }
}

fn shift_enemy (
    mut enemy: Query<&mut Transform, With<Enemy>>,
    mut manager: ResMut<EnemyManager>
) {
    if manager.is_shift {
        manager.direction *= -1.;
        manager.is_shift = false;

        for mut transform in enemy.iter_mut() { transform.translation.y -= constants::ENEMY_SHIFT; }
        for pos in manager.enemy_id.iter_mut() { pos.y -= constants::ENEMY_SHIFT; }
    }
}

fn enemy_shoot (
    mut commands: Commands,
    enemy_manager: Res<EnemyManager>,
    bullet_sprite: Res<bullet::BulletImage>
) {
    for pos in enemy_manager.enemy_id.iter() {
        commands.spawn((
            Sprite { image: bullet_sprite.0.clone(), ..Default::default() },
            Transform::from_translation(*pos).with_scale(Vec3::splat(constants::SCALE_RATIO)),
            bullet::Bullet,
            bullet::Velocity(constants::ENEMY_BULLET_SPEED),
            bullet::EnemyMarker,
            StateScoped(GameState::InGame)
        ));
    }
}
