use bevy::{
    ecs::{
        event::EventWriter,
        system::{Res, ResMut},
    },
    input::{keyboard::KeyCode, ButtonInput},
};

use crate::{
    events::{
        pause_audio_event::PauseAudioEvent, play_audio_event::PlayAudioEvent,
        user_interface_events::PauseMenuEvent,
    },
    resources::game_state::GameState,
};

pub fn player_pause_resume(
    mut game_state: ResMut<GameState>,
    input: Res<ButtonInput<KeyCode>>,
    mut event_pause_resume: EventWriter<PauseMenuEvent>,
    mut event_play_audio: EventWriter<PlayAudioEvent>,
    mut event_pause_audio: EventWriter<PauseAudioEvent>,
) {
    if input.just_pressed(KeyCode::KeyP) {
        game_state.paused = !game_state.paused;
        event_pause_resume.send(PauseMenuEvent {});

        if game_state.paused {
            event_pause_audio.send(PauseAudioEvent {});
        } else {
            event_play_audio.send(PlayAudioEvent {});
        }
    }
}
