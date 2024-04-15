use bevy::ecs::system::Resource;

use crate::assets::sounds::music::{
    combat_music::CombatMusicSound, exploration_music::ExplorationMusicSound,
    menu_music::MenuMusicSound, trading_music::TradingMusicSound,
};

#[derive(Resource)]
pub struct MusicResource {
    pub combat_music: CombatMusicSound,
    pub exploration_music: ExplorationMusicSound,
    pub trading_music: TradingMusicSound,
    pub menu_music: MenuMusicSound,
}
