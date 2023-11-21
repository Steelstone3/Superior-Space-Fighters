use crate::components::space::Space;
use bevy::{
    prelude::{AssetServer, Commands, Res, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle}, render::view::Visibility, ecs::entity::Entity,
};
use rand::random;

pub fn spawn_random_space_background(
    commands: & mut Commands,
    asset_server: & Res<AssetServer>,
    location: Vec3,
) -> Entity {
    let space = Space {
        asset: random(),
        size: Vec2 {
            x: 1920.0,
            y: 1920.0,
        },
    };

    let texture = asset_server.load(space.asset.to_string());

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
        .insert(space).id();

    return space_id;
}
