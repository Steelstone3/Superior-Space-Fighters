use bevy::{
    ecs::{
        event::EventWriter,
        system::{Res, ResMut},
    },
    input::{keyboard::KeyCode, ButtonInput},
};

use crate::{events::user_interface_events::PauseMenuEvent, resources::game_state::GameState};

pub fn player_pause_resume(
    mut game_state: ResMut<GameState>,
    input: Res<ButtonInput<KeyCode>>,
    mut event_pause_resume: EventWriter<PauseMenuEvent>,
) {
    if input.just_pressed(KeyCode::KeyP) {
        game_state.paused = !game_state.paused;
        event_pause_resume.send(PauseMenuEvent {});
    }
}
