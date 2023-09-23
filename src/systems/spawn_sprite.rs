use bevy::{prelude::{Commands, Vec2, AssetServer, Res}, sprite::{SpriteBundle, Sprite}};

pub fn spawn_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
   let texture = asset_server.load("player_character.png");
   
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..Default::default()
        },
        texture,
        ..Default::default()
    });
}
