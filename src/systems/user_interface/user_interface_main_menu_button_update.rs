use bevy::{
    ecs::{
        schedule::NextState,
        system::{Query, ResMut},
    },
    render::color::Color,
    ui::{BackgroundColor, Interaction},
};

use crate::{
    queries::button_interaction_query::{ButtonInteractionFilter, ButtonInteractionMutableQuery},
    states::core_states::GameState,
};

pub fn user_interface_main_menu_button_update(
    mut interaction_query: Query<ButtonInteractionMutableQuery, ButtonInteractionFilter>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for mut interaction in &mut interaction_query {
        match *interaction.interaction {
            Interaction::Pressed => {
                *interaction.background_colour = BackgroundColor(Color::rgb(0.35, 0.75, 0.35));
                interaction.border_colour.0 = Color::RED;
                next_game_state.set(GameState::InGame)
            }
            Interaction::Hovered => {
                *interaction.background_colour = BackgroundColor(Color::rgb(0.25, 0.25, 0.25));
                interaction.border_colour.0 = Color::WHITE;
            }
            Interaction::None => {
                *interaction.background_colour = BackgroundColor(Color::rgb(0.15, 0.15, 0.15));
                interaction.border_colour.0 = Color::BLACK;
            }
        }
    }
}
