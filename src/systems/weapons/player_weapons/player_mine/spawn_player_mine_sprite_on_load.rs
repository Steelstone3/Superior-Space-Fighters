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

use crate::components::weapons::player_weapons::player_mine::PlayerMine;

//When loading space will be loaded without a sprite this will spawn new ones for them
pub fn spawn_player_mine_sprite_on_load(
    spriteless_weapon: Query<(Entity, &PlayerMine, &Transform), Without<Sprite>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for weapon in spriteless_weapon.iter() {
        let texture = asset_server.load(weapon.1.mine.mine.to_string());
        let custom_size = Some(weapon.1.mine.lifetime_weapon.weapon.size);
        let transform = *weapon.2;

        if let Some(mut starship_entity_commands) = commands.get_entity(weapon.0) {
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
