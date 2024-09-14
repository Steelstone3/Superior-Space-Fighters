use bevy::{
    ecs::system::{Commands, Query, ResMut},
    hierarchy::DespawnRecursiveExt,
    prelude::NextState,
};

use crate::{
    queries::main_menu_parent_queries::MainMenuParentEntityQuery, states::core_states::GameState,
};

pub fn start_new_game(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    main_menu_parent_query: Query<MainMenuParentEntityQuery>,
) {
    next_game_state.set(GameState::InGame);

    if let Ok(main_menu_parent) = main_menu_parent_query.get_single() {
        commands.entity(main_menu_parent.entity).despawn_recursive();
    }
}
