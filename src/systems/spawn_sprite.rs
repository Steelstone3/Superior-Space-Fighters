use bevy::{prelude::{Commands, Vec2}, sprite::{SpriteBundle, Sprite}};

pub fn spawn_sprite(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..Default::default()
        },
        ..Default::default()
    });
}
