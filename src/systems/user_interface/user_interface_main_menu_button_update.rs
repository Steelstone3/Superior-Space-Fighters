use bevy::{
    ecs::{
        event::EventWriter,
        query::{Changed, With},
        system::Query,
    },
    render::color::Color,
    ui::{widget::Button, BackgroundColor, BorderColor, Interaction},
};

use crate::events::game_state_events::NewGameEvent;

pub fn user_interface_main_menu_button_update(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut new_game_event_writer: EventWriter<NewGameEvent>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::rgb(0.35, 0.75, 0.35));
                border_color.0 = Color::RED;

                new_game_event_writer.send(NewGameEvent {});
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::rgb(0.25, 0.25, 0.25));
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = BackgroundColor(Color::rgb(0.15, 0.15, 0.15));
                border_color.0 = Color::BLACK;
            }
        }
    }
}
