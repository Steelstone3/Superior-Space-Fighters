use crate::{assets::images::space::SpaceSprite, components::space::Space};
use bevy::{
    math::{Vec2, Vec3},
    prelude::{AssetServer, Commands, Res},
    sprite::{Sprite, SpriteBundle},
};
use rand::random;

pub fn spawn_random_empty_space_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let space = random::<SpaceSprite>();
    let tile_size = 1920.0;

    for x in 0..5 {
        for y in 0..5 {
            let grid_position = Vec2::new(x as f32, y as f32);
            let location = Vec3::new(
                (grid_position.x * tile_size) - tile_size,
                (grid_position.y * tile_size) - tile_size,
                0.0,
            );

            let space = Space::new(space, tile_size, grid_position, location);

            let texture = asset_server.load(space.space.to_string());

            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(space.size),
                        ..Default::default()
                    },
                    texture,
                    transform: bevy::prelude::Transform {
                        translation: space.location,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(space);
        }
    }
}
