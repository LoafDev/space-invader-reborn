use bevy::prelude::*;
use crate::constants;
use super::InGameState;

#[derive(Component)]
struct PauseText;

pub struct PausedPlugin;
impl Plugin for PausedPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(InGameState::Paused), setup_paused)
            .add_systems(Update, pause_text_color.run_if(in_state(InGameState::Paused)));

    }
}

fn setup_paused(mut commands: Commands) {
    commands.spawn((
        Text::new(constants::PAUSE_TEXT),
        TextFont { font_size: 20., ..Default::default() },
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.),
            left: Val::Px(constants::MAX_WIDTH / 2. - 58.),
            ..Default::default()
        },
        PauseText,
        StateScoped(InGameState::Paused)
    ));
}

fn pause_text_color (
    time: Res<Time>,
    mut pause: Single<&mut TextColor, With<PauseText>>
) {
    let seconds = time.elapsed_secs();

    pause.0 = Color::srgb(
        ops::sin(1.25 * seconds) / 2.0 + 0.5,
        ops::sin(0.75 * seconds) / 2.0 + 0.5,
        ops::sin(0.50 * seconds) / 2.0 + 0.5,
    );
}
