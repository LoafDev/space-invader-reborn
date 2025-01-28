use bevy::prelude::*;
use crate::{constants, game::{self, collisions}, GameState};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), setup_score)
            .add_systems(Update, (draw_score, update_score).run_if(in_state(GameState::InGame)));
    }
}

#[derive(Resource, Deref, DerefMut)]
struct Score(usize);

#[derive(Component)]
struct ScoreText;

fn setup_score (mut commands: Commands) {
    commands.insert_resource(Score(0));

    commands.spawn((
        Text::new("Score: "),
        TextFont { font_size: 20., ..Default::default() },
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.),
            right: Val::Px(5.),
            ..Default::default()
        },
        game::WhileInGame
    )).with_child((
        TextSpan::default(),
        TextFont { font_size: 33., ..Default::default() },
        TextColor(constants::TEXT_COLOR),
        ScoreText,
        game::WhileInGame
    ));
}

fn draw_score (
    score: Res<Score>,
    score_text: Single<&mut TextSpan, With<ScoreText>>
) {
    let mut span = score_text.into_inner();
    **span = format!("{}", score.0);
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
