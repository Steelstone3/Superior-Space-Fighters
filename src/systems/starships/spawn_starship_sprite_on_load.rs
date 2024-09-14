use crate::components::starships::starship::Starship;
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

//When loading starships will be loaded without a sprite this will spawn new ones for them
#[allow(dead_code)]
pub fn spawn_starship_sprite_on_load(
    spriteless_starships: Query<(Entity, &Starship, &Transform), Without<Sprite>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for starship in spriteless_starships.iter() {
        let texture = asset_server.load(starship.1.faction_starship.to_string());
        let custom_size = Some(starship.1.size);
        let transform = *starship.2;

        if let Some(mut starship_entity_commands) = commands.get_entity(starship.0) {
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
