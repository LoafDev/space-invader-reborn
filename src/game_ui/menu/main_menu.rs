use bevy::prelude::*;
use crate::{constants, game_ui::{self, menu}};
use super::MenuButton;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(menu::MenuState::InMain), setup_main_menu);
    }
}

fn setup_main_menu (mut commands: Commands, cus_font: Res<game_ui::CusFont>) {
    let button_node = Node {
        width: Val::Px(300.),
        height: Val::Px(65.),
        margin: UiRect::all(Val::Px(20.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };

    let button_font = TextFont { font_size: 40., font: cus_font.0.clone(), ..Default::default() };

    commands.spawn((
       Node {
           width: Val::Percent(100.),
           height: Val::Percent(100.),
           align_items: AlignItems::Center,
           justify_content: JustifyContent::Center,
           ..Default::default()
       }, StateScoped(menu::MenuState::InMain)
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
                Text::new(constants::GAME_NAME),
                TextFont { font_size:  50., font: cus_font.0.clone(), ..Default::default() },
                TextColor(constants::TEXT_COLOR),
                Node { margin: UiRect::all(Val::Px(50.)), ..Default::default() }
            ));

            child.spawn((
                Button,
                button_node.clone(),
                BackgroundColor(constants::BACKGROUND_COLOR),
                MenuButton::Play
            ))
            .with_children(|grandchild| {
                grandchild.spawn((
                    Text::new("Play game!"),
                    button_font.clone(),
                    TextColor(constants::TEXT_COLOR)
                ));
            });

            child.spawn((
                Button,
                button_node.clone(),
                BackgroundColor(constants::BACKGROUND_COLOR),
                MenuButton::Credit
            ))
            .with_children(|grandchild| {
                grandchild.spawn((
                    Text::new("Credit"),
                    button_font.clone(),
                    TextColor(constants::TEXT_COLOR)
                ));
            });

            child.spawn((
                Button,
                button_node.clone(),
                BackgroundColor(constants::BACKGROUND_COLOR),
                MenuButton::Exit
            ))
            .with_children(|grandchild| {
                grandchild.spawn((
                    Text::new("Quit game!"),
                    button_font,
                    TextColor(constants::TEXT_COLOR)
                ));
            });
        });
    });
}

