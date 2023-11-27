use crate::{components::space::Space, assets::images::space::SpaceSprite};
use bevy::{
    math::{Vec2, Vec3},
    prelude::{AssetServer, Commands, Res},
    sprite::{Sprite, SpriteBundle},
};
use rand::random;

pub fn spawn_random_empty_space_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let space_tile_size = 1920.0;
    let total_area_around_player = 1920.0 * 1.5;
    let half_space_tile_size = 1920.0 * 0.5;

    let rand_texture = random::<SpaceSprite>();

    for x in 0..5 {
        for y in 0..5 {
            let space_grid_position = Vec2::new(x as f32, y as f32);

            let space = Space {
                asset: random(),
                size: Vec2 {
                    x: space_tile_size,
                    y: space_tile_size,
                },
                space_grid_position,
                space_location: Vec3::new(
                    (space_grid_position.x * space_tile_size) - total_area_around_player
                        + half_space_tile_size,
                    (space_grid_position.y * space_tile_size) - total_area_around_player
                        + half_space_tile_size,
                    0.0,
                ),
            };

            let texture = asset_server.load(rand_texture.to_string());

            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(space.size),
                        ..Default::default()
                    },
                    texture,
                    transform: bevy::prelude::Transform {
                        translation: space.space_location,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(space);
        }
    }
}
