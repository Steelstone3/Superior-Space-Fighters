use crate::components::space::Space;
use bevy::{
    prelude::{AssetServer, Commands, Res, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
};
use rand::random;

pub fn spawn_random_space_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let space = Space {
        asset: random(),
        size: Vec2 {
            x: 1920.0,
            y: 1920.0,
        },
    };

    let texture = asset_server.load(space.asset.to_string());

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(space.size),
                ..Default::default()
            },
            texture,
            transform: bevy::prelude::Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(space);
}
