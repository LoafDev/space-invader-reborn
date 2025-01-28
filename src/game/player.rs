use std::time::Duration;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use crate::{constants, GameState, game::{bullet, cameras, collisions, self}};

pub struct PlayerPlugin;

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct PlayerShoot {
    ammo: usize,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), setup_player)
            .add_systems(Update, (
                input_player,
                player_shoot,
                player_cooldown.run_if(on_timer(Duration::from_secs(1)))
            ).run_if(in_state(GameState::InGame)));
    }
}

fn setup_player (
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.insert_resource( PlayerShoot { ammo: constants::PLAYER_BULLETS } );
    commands.spawn((
        Sprite { image: asset_server.load("player.png"), ..Default::default() },
        Transform::from_xyz(0., -constants::MAX_HEIGHT*0.5, 0.).with_scale(Vec3::splat(constants::SCALE_RATIO)),
        Player,
        cameras::RotateToMouse,
        collisions::Collider,
        game::WhileInGame
    ));
}

fn input_player (
    mut player: Single<&mut Transform, With<Player>>,
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    let mut direction = 0.;

    if key.pressed(KeyCode::KeyA) { direction = -1.; }
    if key.pressed(KeyCode::KeyD) { direction = 1.; }

    let new_pos = player.translation.x + direction * constants::PLAYER_SPEED * time.delta_secs();

    player.translation.x = new_pos.clamp(constants::MAX_WIDTH*(-0.5), constants::MAX_WIDTH*0.5);
}

fn player_shoot (
    player: Single<&Transform, With<Player>>,
    mut commands: Commands,
    mut player_shoot: ResMut<PlayerShoot>,
    bullet_sprite: Res<bullet::BulletImage>,
    button: Res<ButtonInput<MouseButton>>
) {
    if button.just_pressed(MouseButton::Left) && player_shoot.ammo > 0 {
        commands.spawn((
            Sprite { image: bullet_sprite.0.clone(), ..Default::default() },
            Transform::from_translation(Vec3::new(player.translation.x, player.translation.y, -1.)).with_scale(Vec3::splat(constants::SCALE_RATIO)).with_rotation(player.rotation),
            bullet::Bullet,
            bullet::Velocity,
            game::WhileInGame
        ));
        player_shoot.ammo -= 1;
    }
}

fn player_cooldown (
    mut player_shoot: ResMut<PlayerShoot>
) {
    if player_shoot.ammo < constants::PLAYER_BULLETS { player_shoot.ammo += 1; }
}
