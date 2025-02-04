use bevy::{math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume}, prelude::*};
use crate::{ constants, game::{ bullet, enemy, InGameState } };

#[derive(Component)]
pub struct Collider;

#[derive(Event, Default)]
pub struct BulletColide;

#[derive(Event, Default)]
pub struct Collision(pub bool);

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Collision>()
            .add_event::<BulletColide>()
            .add_systems(Update, (check_bullet_colide, check_collision).run_if(in_state(InGameState::Playing)));
    }
}


fn isbulletcolide (
    bullet: BoundingCircle,
    enemy_bullet: BoundingCircle
) -> bool { if bullet.intersects(&enemy_bullet) { true } else { false } }

fn check_bullet_colide (
    mut commands: Commands,
    bullet: Query<(Entity, &Transform), (With<bullet::Velocity>, Without<bullet::EnemyMarker>)>,
    enemy_bullet: Query<(Entity, &Transform), (With<bullet::EnemyMarker>, With<bullet::Velocity>)>,
    mut event: EventWriter<BulletColide>
) {
    for (entity, transform) in bullet.iter() {
        for (enemy_entity, enemy_transform) in enemy_bullet.iter() {
            if isbulletcolide (
                BoundingCircle::new(transform.translation.truncate(), constants::BULLET_RADIUS),
                BoundingCircle::new(enemy_transform.translation.truncate(), constants::BULLET_RADIUS)
                ) {
                commands.entity(entity).despawn();
                commands.entity(enemy_entity).despawn();
                event.send_default();
            }
        }
    }
}

fn iscollision (
    bullet: BoundingCircle,
    collider: Aabb2d
) -> bool { if bullet.intersects(&collider) { true } else { false } }

fn check_collision (
    mut commands: Commands,
    bullet_q: Query<(Entity, &Transform, Option<&bullet::EnemyMarker>), With<bullet::Velocity>>,
    collider_q: Query<(Entity, &Transform, Option<&enemy::Enemy>), With<Collider>>,
    mut collision_event: EventWriter<Collision>
) {
    for (bullet_entity, transform, maybe_enemy_bullet) in &bullet_q {
        for (entity, entity_transform, maybe_enemy) in &collider_q {
            if iscollision (
                BoundingCircle::new(transform.translation.truncate(), constants::BULLET_RADIUS),
                Aabb2d::new(entity_transform.translation.truncate(), entity_transform.scale.truncate() / 2.)
            ) {
                if maybe_enemy.is_some() && maybe_enemy_bullet.is_none() {
                    commands.entity(entity).despawn();
                    commands.entity(bullet_entity).despawn();
                    collision_event.send(Collision(true));
                } else if maybe_enemy.is_none() && maybe_enemy_bullet.is_some() {
                    commands.entity(bullet_entity).despawn();
                    collision_event.send(Collision(false));
                }
            }
        }
    }
}

