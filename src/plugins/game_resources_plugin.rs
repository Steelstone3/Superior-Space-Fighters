use crate::{
    assets::sounds::music::{
        combat_music::CombatMusicSound, exploration_music::ExplorationMusicSound,
        menu_music::MenuMusicSound, trading_music::TradingMusicSound,
    },
    resources::{
        camera_settings::CameraSettings, fleet_credits::FleetCredits, music::Music,
        projectile_ammunition::ProjectileAmmunition, sector_size::SectorSize,
        targetting_settings::TargettingSettings,
    },
};
use bevy::prelude::{App, Plugin};

pub struct GameResourcesPlugin;

impl Plugin for GameResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TargettingSettings {
            maximum_distance: 2000.0,
            is_targetting: false,
        })
        .insert_resource(SectorSize {
            top_border: 1000.0,
            left_border: -1000.0,
            bottom_border: -1000.0,
            right_border: 1000.0,
        })
        .insert_resource(FleetCredits(1000))
        .insert_resource(ProjectileAmmunition {
            blaster_ammunition: 20,
            torpedo_ammunition: 5,
            mine_ammunition: 7,
            exotic_ammunition: 2,
            selected_weapon: 1,
        })
        .insert_resource(CameraSettings {
            zoom_speed: 1.5,
            minimum_zoom: 0.5,
            maximum_zoom: 2.0,
            current_zoom: 1.0,
            zoom_in: 1.1,
            zoom_out: 0.9,
        })
        .insert_resource(Music {
            combat_music: CombatMusicSound::default(),
            exploration_music: ExplorationMusicSound::default(),
            trading_music: TradingMusicSound::default(),
            menu_music: MenuMusicSound::default(),
        });
    }
}
