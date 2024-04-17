use bevy::{
    ecs::{
        event::EventReader,
        schedule::NextState,
        system::{Commands, Query, ResMut},
    },
    hierarchy::DespawnRecursiveExt,
};

use crate::{
    events::game_state_events::NewGameEvent,
    queries::main_menu_parent_queries::MainMenuParentEntityQuery, states::core_states::GameState,
};

pub fn start_new_game(
    mut new_game_event_reader: EventReader<NewGameEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    main_menu_parent_query: Query<MainMenuParentEntityQuery>,
) {
    next_state.set(GameState::InGame);

    for _ in new_game_event_reader.read() {
        if let Ok(main_menu_parent) = main_menu_parent_query.get_single() {
            commands.entity(main_menu_parent.entity).despawn_recursive();
        }
    }
}
