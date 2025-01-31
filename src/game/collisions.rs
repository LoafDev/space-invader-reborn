use bevy::{math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume}, prelude::*};
use crate::{ constants, game::{ bullet, enemy, InGameState } };

#[derive(Component)]
pub struct Collider;

#[derive(Event, Default)]
pub struct Collision(pub bool);

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Collision>()
            .add_systems(Update, check_collision.run_if(in_state(InGameState::Playing)));
    }
}

fn iscollision (
    bullet: BoundingCircle,
    collider: Aabb2d
) -> bool { if bullet.intersects(&collider) { true } else { false } }


fn check_collision (
    mut commands: Commands,
    bullet_q: Query<(Entity, &Transform, Option<&bullet::RNGSpawnRate>), With<bullet::Velocity>>,
    collider_q: Query<(Entity, &Transform, Option<&enemy::Enemy>), With<Collider>>,
    mut collision_event: EventWriter<Collision>
) {
    for (bullet_entity, transform, maybe_enemy_bullet) in &bullet_q {
        for (entity, entity_transform, maybe_enemy) in &collider_q {
            let is_collision = iscollision(
                BoundingCircle::new(transform.translation.truncate(), constants::BULLET_RADIUS),
                Aabb2d::new(entity_transform.translation.truncate(), entity_transform.scale.truncate() / 2.)
            );

            if is_collision {
                if maybe_enemy.is_some() && maybe_enemy_bullet.is_none() {
                    commands.entity(entity).despawn();
                    commands.entity(bullet_entity).despawn();
                    collision_event.send(Collision(true));
                } else if maybe_enemy.is_none() && maybe_enemy_bullet.is_some() {
                    println!("Player is dead but I'm too lazy to implement game over screen lol");
                    commands.entity(bullet_entity).despawn();
                    collision_event.send(Collision(false));
                }
            }
        }
    }
}

