use bevy::ecs::event::EventWriter;

use crate::events::user_interface_events::MainMenuEvent;

pub fn draw_main_menu(mut draw_ui_event: EventWriter<MainMenuEvent>) {
    draw_ui_event.send(MainMenuEvent {});
}
