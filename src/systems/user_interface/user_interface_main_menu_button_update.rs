use bevy::{
    ecs::{
        event::EventWriter,
        schedule::NextState,
        system::{Query, ResMut},
    },
    render::color::Color,
    ui::{BackgroundColor, Interaction},
};

use crate::{
    events::{
        logging_event::LoggingEvent,
        user_interface_events::{LoadGameEvent, NewGameEvent, SaveGameEvent},
    },
    queries::button_interaction_query::{ButtonInteractionFilter, ButtonInteractionMutableQuery},
    states::core_states::GameState,
};

pub fn user_interface_menu_button_update(
    mut interaction_query: Query<ButtonInteractionMutableQuery, ButtonInteractionFilter>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_game_event_writer: EventWriter<NewGameEvent>,
    mut logging_event_writer: EventWriter<LoggingEvent>,
    mut load_game_event: EventWriter<LoadGameEvent>,
    mut save_game_event: EventWriter<SaveGameEvent>,
) {
    for mut interaction in &mut interaction_query {
        match *interaction.interaction {
            Interaction::Pressed => {
                *interaction.background_colour = BackgroundColor(Color::rgb(0.35, 0.75, 0.35));
                interaction.border_colour.0 = Color::RED;
                if let Some(_) = interaction.new_game_button {
                    logging_event_writer.send(LoggingEvent {
                        message: "Starting New Game".to_string(),
                    });
                    next_game_state.set(GameState::InGame);
                    next_game_event_writer.send(NewGameEvent);
                } else if let Some(_) = interaction.save_game_button {
                    logging_event_writer.send(LoggingEvent {
                        message: "Saving Game".to_string(),
                    });
                    save_game_event.send(SaveGameEvent);
                } else if let Some(_) = interaction.load_game_button {
                    logging_event_writer.send(LoggingEvent {
                        message: "Loading Game".to_string(),
                    });
                    load_game_event.send(LoadGameEvent);
                }
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
