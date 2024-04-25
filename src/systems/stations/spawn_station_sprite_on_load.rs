use bevy::{
    asset::AssetServer,
    ecs::{
        entity::Entity,
        query::Without,
        system::{Commands, Query, Res},
    },
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};

use crate::components::station::SpaceStation;

//When loading space will be loaded without a sprite this will spawn new ones for them
pub fn spawn_station_sprite_on_load(
    spriteless_space: Query<(Entity, &SpaceStation, &Transform), Without<Sprite>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for space in spriteless_space.iter() {
        let texture = asset_server.load(space.1.station.to_string());
        let custom_size = Some(space.1.size);
        let transform = *space.2;

        if let Some(mut starship_entity_commands) = commands.get_entity(space.0) {
            starship_entity_commands.insert(SpriteBundle {
                sprite: Sprite {
                    custom_size,
                    ..Default::default()
                },
                texture,
                transform,
                ..Default::default()
            });
        };
    }
}
