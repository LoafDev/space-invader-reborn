use bevy::prelude::*;
use crate::game_ui::menu;

#[derive(Component, Debug)]
struct WhileInCredit;

pub struct CreditPlugin;
impl Plugin for CreditPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(menu::MenuState::InCredit), say_hello)
            .add_systems(OnExit(menu::MenuState::InCredit), crate::despawn_screen::<WhileInCredit>);
    }
}

fn say_hello (
    query: Query<&WhileInCredit>
) {
    for q in &query { println!("Hello! It's me! {:?}", q); }
}
