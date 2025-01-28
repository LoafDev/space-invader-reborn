// Text constants
pub const GAME_NAME: &str = "Space invader game or something lol!";

// Window and scale constants
pub const MAX_HEIGHT: f32 = 512.;
pub const MAX_WIDTH: f32 = 512.;
pub const SCALE_RATIO: f32 = 2.;

// UI color constants
pub const TEXT_COLOR: bevy::prelude::Color = bevy::color::Color::srgb(0.1,0.1,0.4);
pub const BACKGROUND_COLOR: bevy::prelude::Color = bevy::color::Color::srgb(0.5,0.5,0.5);

// Enemies' constants
pub const WIDTH_ENEMY: usize = 10;
pub const HEIGHT_ENEMY: usize = 5;
pub const ENEMY_SPACE: f32 = 24.;
pub const ENEMY_SPEED: f32 = 100.;
pub const ENEMY_SHIFT: f32 = 30.;

// Player's constants
pub const PLAYER_SPEED: f32 = 200.;


// Bullets' constants
pub const BULLET_RADIUS: f32 = 20.;
pub const PLAYER_BULLETS: usize = 100; //for testing purpose, default is 2
pub const PLAYER_BULLET_SPEED: f32 = 500.;
pub const ENEMY_BULLETS: usize = 1;
pub const ENEMY_BULLET_SPEED: f32 = 100.;
 
