use bevy::{prelude::*, window::PrimaryWindow};
use crate::game::InGameState;
use ops::atan2;

#[derive(Resource, Default)]
pub struct WorldCoord(pub Vec2);

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct RotateToMouse;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WorldCoord>()
            .add_systems(Startup, setup_camera)
            .add_systems(Update, (cursor_system, rotate_to_mouse).run_if(in_state(InGameState::Playing)));
    }
}

fn setup_camera (
    mut commands: Commands
) { commands.spawn((Camera2d, MainCamera)); }


fn cursor_system (
    mut coords: ResMut<WorldCoord>,
    cam_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    win_query: Query<&Window, With<PrimaryWindow>>
) {
    let (camera, camera_transform) = cam_query.single();
    let window = win_query.single();

    if let Some(pos) = window.cursor_position()
        .and_then(|cursor| Some(camera.viewport_to_world(camera_transform, cursor)))
        .map(|ray| ray.unwrap().origin.truncate()) { coords.0 = pos; }
}

fn rotate_to_mouse(
    mut obj: Single<&mut Transform, With<RotateToMouse>>,
    coords: Res<WorldCoord>
) {
    let mouse_distant = Vec2::new(obj.translation.x - coords.0.x, coords.0.y - obj.translation.y);
    let angle_mouse = atan2(mouse_distant.x, mouse_distant.y);
    obj.rotation = Quat::from_rotation_z(angle_mouse);
}

