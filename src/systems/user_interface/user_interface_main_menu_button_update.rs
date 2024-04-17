use bevy::{
    ecs::{event::EventWriter, system::Query},
    render::color::Color,
    ui::{BackgroundColor, Interaction},
};

use crate::{
    events::game_state_events::NewGameEvent,
    queries::button_interaction_query::{ButtonInteractionFilter, ButtonInteractionMutableQuery},
};

pub fn user_interface_main_menu_button_update(
    mut interaction_query: Query<ButtonInteractionMutableQuery, ButtonInteractionFilter>,
    mut new_game_event_writer: EventWriter<NewGameEvent>,
) {
    for mut interaction in &mut interaction_query {
        match *interaction.interaction {
            Interaction::Pressed => {
                *interaction.background_colour = BackgroundColor(Color::rgb(0.35, 0.75, 0.35));
                interaction.border_colour.0 = Color::RED;

                new_game_event_writer.send(NewGameEvent {});
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
