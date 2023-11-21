use crate::components::space::Space;
use bevy::{
    ecs::entity::Entity,
    prelude::{AssetServer, Commands, Res, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
};
use rand::random;

pub fn spawn_random_space_background(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    location: Vec3,
    texture_location: Option<String>,
) -> (Entity, String) {
    let space = Space {
        asset: random(),
        size: Vec2 {
            x: 1920.0,
            y: 1920.0,
        },
    };

    let texture_location = texture_location.unwrap_or(space.asset.to_string());
    let texture = asset_server.load(&texture_location);

    let space_id = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(space.size),
                ..Default::default()
            },
            texture,
            transform: bevy::prelude::Transform {
                translation: location,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(space)
        .id();

    return (space_id, texture_location);
}
