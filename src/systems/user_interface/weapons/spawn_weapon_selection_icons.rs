use crate::components::user_interface::weapon_icon::WeaponIcon;
use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
    math::{Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
    utils::tracing,
};

#[allow(dead_code)]
pub fn spawn_weapon_selection_icons(mut commands: Commands, asset_server: Res<AssetServer>) {
    tracing::info!("Spawn selected blaster icon");
    let weapon_icon = WeaponIcon::new_unselected_blaster_icon();
    let texture = asset_server.load(weapon_icon.icon.to_string());

    commands
        .spawn(weapon_icon_sprite_bundle(
            &weapon_icon,
            texture,
            Vec2::new(100.0, 100.0),
        ))
        .insert(weapon_icon);

    tracing::info!("Spawn selected torpedo icon");
    let weapon_icon = WeaponIcon::new_unselected_torpedo_icon();
    let texture = asset_server.load(weapon_icon.icon.to_string());

    commands
        .spawn(weapon_icon_sprite_bundle(
            &weapon_icon,
            texture,
            Vec2::new(200.0, 100.0),
        ))
        .insert(weapon_icon);

    tracing::info!("Spawn selected mine icon");
    let weapon_icon = WeaponIcon::new_unselected_mine_icon();
    let texture = asset_server.load(weapon_icon.icon.to_string());

    commands
        .spawn(weapon_icon_sprite_bundle(
            &weapon_icon,
            texture,
            Vec2::new(300.0, 100.0),
        ))
        .insert(weapon_icon);

    tracing::info!("Spawn selected exotic icon");
    let weapon_icon = WeaponIcon::new_unselected_exotic_icon();
    let texture = asset_server.load(weapon_icon.icon.to_string());

    commands
        .spawn(weapon_icon_sprite_bundle(
            &weapon_icon,
            texture,
            Vec2::new(400.0, 100.0),
        ))
        .insert(weapon_icon);
}

#[allow(dead_code)]
fn weapon_icon_sprite_bundle(
    weapon_icon: &WeaponIcon,
    texture: bevy::prelude::Handle<bevy::prelude::Image>,
    position: Vec2,
) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            custom_size: Some(weapon_icon.size),
            ..Default::default()
        },
        texture,
        transform: bevy::prelude::Transform {
            translation: Vec3::new(position.x, position.y, 100.0),
            ..Default::default()
        },
        ..Default::default()
    }
}
