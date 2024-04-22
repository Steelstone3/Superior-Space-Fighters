use bevy::{
    ecs::{
        event::{EventReader, EventWriter},
        schedule::{NextState, State},
        system::{Res, ResMut},
    },
    input::{keyboard::KeyCode, ButtonInput},
};

use crate::{
    events::{
        audio_events::{PauseAudioEvent, PlayAudioEvent},
        user_interface_events::PauseEvent,
    },
    states::core_states::GameState,
};

pub fn player_pause_resume(
    input: Res<ButtonInput<KeyCode>>,
    mut play_audio_event_writer: EventWriter<PlayAudioEvent>,
    mut pause_audio_event_writer: EventWriter<PauseAudioEvent>,
    current_game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut pause_event_reader: EventReader<PauseEvent>,
) {
    if input.just_pressed(KeyCode::KeyP) {
        let current_game_state = current_game_state.get();
        if current_game_state == &GameState::InGame {
            next_game_state.set(GameState::PauseMenu);
            pause_audio_event_writer.send(PauseAudioEvent {});
        } else if current_game_state == &GameState::PauseMenu {
            next_game_state.set(GameState::InGame);
            play_audio_event_writer.send(PlayAudioEvent {});
        }
    }

    for pause_event in pause_event_reader.read() {
        if pause_event.pause {
            next_game_state.set(GameState::PauseMenu);
            pause_audio_event_writer.send(PauseAudioEvent {});
        } else {
            next_game_state.set(GameState::InGame);
            play_audio_event_writer.send(PlayAudioEvent {});
        }
    }
}
