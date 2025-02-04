use bevy::prelude::*;
use crate::{constants, game::{collisions, InGameState}, game_ui, GameState};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), setup_score)
            .add_systems(Update, (draw_score, update_score).run_if(in_state(InGameState::Playing)));
    }
}

#[derive(Resource, Deref, DerefMut)]
struct Score(usize);

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct AmmoText;

fn setup_score (mut commands: Commands, cus_font: Res<game_ui::CusFont>) {
    commands.insert_resource(Score(0));

    commands.spawn((
        Text::new("Score: "),
        TextFont { font_size: 40., font: cus_font.0.clone(), ..Default::default() },
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.),
            left: Val::Px(5.),
            ..Default::default()
        },
        StateScoped(GameState::InGame)
    )).with_child((
        TextSpan::default(),
        TextFont { font_size: 39., font: cus_font.0.clone(), ..Default::default() },
        TextColor(constants::TEXT_COLOR),
        ScoreText
    ));

    commands.spawn((
        Text::new("Ammo: "),
        TextFont { font_size: 40., font: cus_font.0.clone(), ..Default::default() },
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.),
            right: Val::Px(5.),
            ..Default::default()
        },
        StateScoped(GameState::InGame)
    )).with_child((
        TextSpan::default(),
        TextFont { font_size: 39., font: cus_font.0.clone(), ..Default::default() },
        TextColor(constants::TEXT_COLOR),
        AmmoText
    ));

}

fn draw_score (
    score: Res<Score>,
    ammo: Res<crate::game::player::PlayerEssentials>,
    score_text: Single<&mut TextSpan, (With<ScoreText>, Without<AmmoText>)>,
    ammo_text: Single<&mut TextSpan, With<AmmoText>>
) {
    let mut score_span = score_text.into_inner();
    let mut ammo_span = ammo_text.into_inner();

    **score_span = format!("{}", score.0);
    **ammo_span = format!("{}", ammo.ammo);
}

fn update_score (
    mut score: ResMut<Score>,
    mut collision_receive: EventReader<collisions::Collision>,
    mut gamestate: ResMut<NextState<GameState>>,
) {
    for b in collision_receive.read() {
        if b.0 {
            **score += 1;
             if score.0 == constants::HEIGHT_ENEMY * constants::WIDTH_ENEMY { gamestate.set(GameState::GameWin); }
        }
    }
}
