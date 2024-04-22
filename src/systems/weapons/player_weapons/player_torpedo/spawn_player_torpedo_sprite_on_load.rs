use bevy::{
    asset::AssetServer,
    ecs::{
        query::Without,
        system::{Commands, Query, Res},
    },
    sprite::{Sprite, SpriteBundle},
};

use crate::queries::{
    no_sprite_query::NoSpriteFilter, player_torpedo_queries::PlayerTorpedoEntityTransformQuery,
};

//When loading space will be loaded without a sprite this will spawn new ones for them
pub fn spawn_player_torpedo_sprite_on_load(
    spriteless_weapon: Query<PlayerTorpedoEntityTransformQuery, NoSpriteFilter>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for weapon in spriteless_weapon.iter() {
        let texture = asset_server.load(weapon.player_torpedo.torpedo.torpedo.to_string());
        let custom_size = Some(
            weapon
                .player_torpedo
                .torpedo
                .lock_on_weapon
                .ranged_weapon
                .weapon
                .size,
        );
        let transform = *weapon.transform;

        if let Some(mut starship_entity_commands) = commands.get_entity(weapon.entity) {
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
