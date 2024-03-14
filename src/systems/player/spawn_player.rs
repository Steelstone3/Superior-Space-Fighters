use bevy::{
    prelude::{AssetServer, Commands, Res, Vec3},
    sprite::{Sprite, SpriteBundle},
};

use crate::components::starships::player_starship::PlayerStarship;

pub fn spawn_player_ship(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player = PlayerStarship::default();

    let texture = asset_server.load(player.ship.faction_starship.to_string());

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(player.ship.size),
                ..Default::default()
            },
            texture,
            transform: bevy::prelude::Transform {
                translation: Vec3::new(0.0, 0.0, 4.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(player);
}
