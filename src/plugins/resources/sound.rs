use bevy::app::Plugin;

use crate::{
    assets::sounds::music::{
        combat_music::CombatMusicSound, exploration_music::ExplorationMusicSound,
        menu_music::MenuMusicSound, trading_music::TradingMusicSound,
    },
    resources::music::Music,
};

pub struct Sound;

impl Plugin for Sound {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Music {
            combat_music: CombatMusicSound::default(),
            exploration_music: ExplorationMusicSound::default(),
            trading_music: TradingMusicSound::default(),
            menu_music: MenuMusicSound::default(),
        });
    }
}
