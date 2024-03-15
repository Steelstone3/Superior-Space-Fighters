use crate::components::weapons::weapon_types::targetting_setting::TargettingSettings;
use bevy::{
    prelude::Commands,
    sprite::{Sprite, SpriteBundle},
};

pub fn spawn_targetting_setting(mut commands: Commands) {
    let targetting_setting = TargettingSettings::default();

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(targetting_setting);
}
