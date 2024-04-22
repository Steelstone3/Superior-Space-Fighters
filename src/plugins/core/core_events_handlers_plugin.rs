use crate::{
    states::core_states::GameState,
    systems::{
        core::{
            despawn_on_return_to_menu::despawn_on_return_to_menu, start_new_game::start_new_game,
        },
        event_handlers::logging_event_handlers::logging,
    },
};
use bevy::{
    app::{Plugin, Update},
    ecs::schedule::OnEnter,
};

pub struct CoreEventHandlersPlugin;

impl Plugin for CoreEventHandlersPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, logging)
            .add_systems(OnEnter(GameState::InGame), start_new_game)
            .add_systems(OnEnter(GameState::MainMenu), despawn_on_return_to_menu);
    }
}
