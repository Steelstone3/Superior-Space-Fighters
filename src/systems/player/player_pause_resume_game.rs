use bevy::{
    ecs::{
        event::EventWriter,
        schedule::{NextState, State},
        system::{Res, ResMut},
    },
    input::{keyboard::KeyCode, ButtonInput},
};

use crate::{
    events::audio_events::{PauseAudioEvent, PlayAudioEvent},
    states::core_states::GameState,
};

pub fn player_pause_resume(
    input: Res<ButtonInput<KeyCode>>,
    mut event_play_audio: EventWriter<PlayAudioEvent>,
    mut event_pause_audio: EventWriter<PauseAudioEvent>,
    current_game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::KeyP) {
        let current_game_state = current_game_state.get();
        if current_game_state == &GameState::InGame {
            next_game_state.set(GameState::Paused);
            event_pause_audio.send(PauseAudioEvent {});
        } else if current_game_state == &GameState::Paused {
            next_game_state.set(GameState::InGame);
            event_play_audio.send(PlayAudioEvent {});
        }
    }
}
