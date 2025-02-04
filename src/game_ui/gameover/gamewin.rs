use bevy::prelude::*;
use crate::{constants, game_ui::{self, MenuButton}, GameState};

pub struct GameWinPlugin;
impl Plugin for GameWinPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::GameWin), setup_win_menu);
    }
}

fn setup_win_menu (mut commands: Commands, cus_font: Res<game_ui::CusFont>) {
    let button_node = Node {
        width: Val::Px(300.),
        height: Val::Px(65.),
        margin: UiRect::all(Val::Px(20.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };

    let button_font = TextFont { font_size: 33., font: cus_font.0.clone(), ..Default::default() };

    commands.spawn((
       Node {
           width: Val::Percent(100.),
           height: Val::Percent(100.),
           align_items: AlignItems::Center,
           justify_content: JustifyContent::Center,
           ..Default::default()
       }, StateScoped(GameState::GameWin)
    ))
    .with_children(|manager| {
        manager.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BackgroundColor(constants::BACKGROUND_COLOR)
        ))
        .with_children(|child| {
            child.spawn((
                Button,
                button_node.clone(),
                BackgroundColor(constants::BACKGROUND_COLOR),
                MenuButton::Back
            ))
            .with_children(|grandchild| {
                grandchild.spawn((
                    Text::new("Back to menu!"),
                    button_font,
                    TextColor(constants::TEXT_COLOR)
                ));
            });
        });
    });
}

