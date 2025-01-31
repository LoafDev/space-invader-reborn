use bevy::prelude::*;
use crate::{ constants, game_ui::{MenuButton, menu} };

pub struct CreditPlugin;
impl Plugin for CreditPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(menu::MenuState::InCredit), setup_credit);
    }
}

fn setup_credit(mut commands: Commands) {
    let button_node = Node {
        width: Val::Px(300.),
        height: Val::Px(65.),
        margin: UiRect::all(Val::Px(20.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };

    let button_font = TextFont { font_size: 33., ..Default::default() };

    commands.spawn((
       Node {
           width: Val::Percent(100.),
           height: Val::Percent(100.),
           align_items: AlignItems::Center,
           justify_content: JustifyContent::Center,
           ..Default::default()
       }, StateScoped(menu::MenuState::InCredit)
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

