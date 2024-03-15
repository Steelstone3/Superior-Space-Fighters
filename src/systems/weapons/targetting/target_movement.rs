use crate::components::{
    starships::{player_starship::PlayerStarship, starship::Starship},
    weapons::weapon_types::{target::Target, targetting_setting::TargettingSettings},
};
use bevy::prelude::{Commands, Query, Transform};

#[allow(dead_code)]
pub fn target_movement(
    _commands: Commands,
    targetting_settings: Query<&TargettingSettings>,
    _targets: Query<(&mut Transform, &Target)>,
    _player_starships: Query<(&Transform, &PlayerStarship)>,
    _starships: Query<(&Transform, &Starship)>,
) {
    let Ok(_targetting_setting) = targetting_settings.get_single() else {
        return;
    };
}
