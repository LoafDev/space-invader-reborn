use std::time::Duration;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use crate::{constants, game::{bullet, cameras, collisions, InGameState}, GameState};

pub struct PlayerPlugin;

#[derive(Component)]
struct Player;

#[derive(Resource)]
pub struct PlayerEssentials {
    pub ammo: usize,
    pub health: usize
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), setup_player)
            .add_systems(Update, (
                input_player,
                player_shoot,
                player_collision,
                player_cooldown.run_if(on_timer(Duration::from_secs(constants::PLAYER_SHOOT_COOLDOWN)))
            ).run_if(in_state(InGameState::Playing)));
    }
}

fn setup_player (
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.insert_resource( PlayerEssentials { ammo: constants::PLAYER_BULLETS, health: constants::MAX_PLAYER_HEALTH } );
    commands.spawn((
        Sprite { image: asset_server.load("player.png"), ..Default::default() },
        Transform::from_xyz(0., -constants::MAX_HEIGHT*0.5, 0.).with_scale(Vec3::splat(constants::SCALE_RATIO)),
        Player,
        cameras::RotateToMouse,
        collisions::Collider,
        StateScoped(GameState::InGame)
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
    mut player_essentials: ResMut<PlayerEssentials>,
    bullet_sprite: Res<bullet::BulletImage>,
    button: Res<ButtonInput<MouseButton>>
) {
    if button.just_pressed(MouseButton::Left) && player_essentials.ammo > 0 {
        commands.spawn((
            Sprite { image: bullet_sprite.0.clone(), ..Default::default() },
            Transform::from_translation(Vec3::new(player.translation.x, player.translation.y, -1.)).with_scale(Vec3::splat(constants::SCALE_RATIO)).with_rotation(player.rotation),
            bullet::Bullet,
            bullet::Velocity(constants::PLAYER_BULLET_SPEED),
            StateScoped(GameState::InGame)
        ));
        player_essentials.ammo -= 1;
    }
}

fn player_cooldown (
    mut player_shoot: ResMut<PlayerEssentials>
) {
    if player_shoot.ammo < constants::PLAYER_BULLETS { player_shoot.ammo += 1; }
}

fn player_collision (
    mut player_essentials: ResMut<PlayerEssentials>,
    mut collision_receive: EventReader<collisions::Collision>,
    mut gamestate: ResMut<NextState<GameState>>,
) {
    for b in collision_receive.read() {
        if !b.0 {
            player_essentials.health -= constants::ENEMY_DAMAGE;
            if player_essentials.health == 0 { gamestate.set(GameState::GameLost); }
        }
    }
}
