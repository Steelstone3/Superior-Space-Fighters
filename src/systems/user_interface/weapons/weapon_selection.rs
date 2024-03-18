use crate::{
    assets::images::user_interface::weapon_selected::WeaponSelectedIcon,
    components::user_interface::weapon_icon::{self, WeaponIcon},
    systems::user_interface::weapons::spawn_weapon_selection_icons::spawn_weapon_selection_icons,
};
use bevy::{
    asset::AssetServer,
    ecs::{
        entity::Entity,
        system::{Commands, Query, Res},
    },
    input::{keyboard::KeyCode, ButtonInput},
    math::{Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
    utils::tracing,
};

pub fn weapon_selection(
    mut commands: Commands,
    weapon_icons: Query<(Entity, &WeaponIcon)>,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if !input.just_pressed(KeyCode::Numpad1)
        || !input.just_pressed(KeyCode::Numpad2)
        || !input.just_pressed(KeyCode::Numpad3)
        || !input.just_pressed(KeyCode::Numpad4)
    {
        return;
    }

    tracing::info!("Reset weapon selected icon UI");
    for weapon_icon in weapon_icons.into_iter() {
        if weapon_icon.1.icon == WeaponSelectedIcon::BlastersSelected {
            tracing::info!("Despawn selected blaster icon");
            commands.entity(weapon_icon.0).despawn();
            
            tracing::info!("Spawn unselected blaster icon");
            let weapon_icon = WeaponIcon::new_unselected_blaster_icon();
            let texture = asset_server.load(weapon_icon.icon.to_string());    

            commands
            .spawn(weapon_icon_sprite_bundle(
                &weapon_icon,
                texture,
                Vec2::new(100.0, 100.0),
            ))
            .insert(weapon_icon);
        }
        if weapon_icon.1.icon == WeaponSelectedIcon::TorpedoesSelected {
            tracing::info!("Despawn selected torpedo icon");
            commands.entity(weapon_icon.0).despawn();
            
            tracing::info!("Spawn unselected torpedo icon");
            let weapon_icon = WeaponIcon::new_unselected_torpedo_icon();
            let texture = asset_server.load(weapon_icon.icon.to_string());    

            commands
            .spawn(weapon_icon_sprite_bundle(
                &weapon_icon,
                texture,
                Vec2::new(200.0, 100.0),
            ))
            .insert(weapon_icon);
        }
        if weapon_icon.1.icon == WeaponSelectedIcon::MinesSelected {
            tracing::info!("Despawn selected mine icon");
            commands.entity(weapon_icon.0).despawn();
            
            tracing::info!("Spawn unselected mine icon");
            let weapon_icon = WeaponIcon::new_unselected_mine_icon();
            let texture = asset_server.load(weapon_icon.icon.to_string());    

            commands
            .spawn(weapon_icon_sprite_bundle(
                &weapon_icon,
                texture,
                Vec2::new(300.0, 100.0),
            ))
            .insert(weapon_icon);
        }
        if weapon_icon.1.icon == WeaponSelectedIcon::ExoticsSelected {
            tracing::info!("Despawn selected mine icon");
            commands.entity(weapon_icon.0).despawn();
            
            tracing::info!("Spawn unselected mine icon");
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
    }

    if input.just_pressed(KeyCode::Numpad1) {
        tracing::info!("Despawn unselected blaster icon");
        for weapon_icon in weapon_icons.into_iter() {
            if weapon_icon.1.icon == WeaponSelectedIcon::BlastersUnselected {
                commands.entity(weapon_icon.0).despawn();
            }
        }

        tracing::info!("Spawn selected blaster icon");
        let weapon_icon = WeaponIcon::new_selected_blaster_icon();
        let texture = asset_server.load(weapon_icon.icon.to_string());


        commands
            .spawn(weapon_icon_sprite_bundle(
                &weapon_icon,
                texture,
                Vec2::new(100.0, 100.0),
            ))
            .insert(weapon_icon);
    }

    if input.just_pressed(KeyCode::Numpad2) {
        tracing::info!("Despawn unselected torpedo icon");
        for weapon_icon in weapon_icons.into_iter() {
            if weapon_icon.1.icon == WeaponSelectedIcon::TorpedoesUnselected {
                commands.entity(weapon_icon.0).despawn();
            }
        }

        tracing::info!("Spawn selected torpedo icon");
        let weapon_icon = WeaponIcon::new_selected_torpedo_icon();
        let texture = asset_server.load(weapon_icon.icon.to_string());

        commands
            .spawn(weapon_icon_sprite_bundle(
                &weapon_icon,
                texture,
                Vec2::new(200.0, 100.0),
            ))
            .insert(weapon_icon);
    }

    if input.just_pressed(KeyCode::Numpad3) {
        tracing::info!("Despawn unselected mine icon");
        for weapon_icon in weapon_icons.into_iter() {
            if weapon_icon.1.icon == WeaponSelectedIcon::MinesUnselected {
                commands.entity(weapon_icon.0).despawn();
            }
        }

        tracing::info!("Spawn selected mine icon");
        let weapon_icon = WeaponIcon::new_selected_mine_icon();
        let texture = asset_server.load(weapon_icon.icon.to_string());

        commands
            .spawn(weapon_icon_sprite_bundle(
                &weapon_icon,
                texture,
                Vec2::new(200.0, 100.0),
            ))
            .insert(weapon_icon);
    }
    if input.just_pressed(KeyCode::Numpad4) {
        tracing::info!("Despawn unselected exotic icon");
        for weapon_icon in weapon_icons.into_iter() {
            if weapon_icon.1.icon == WeaponSelectedIcon::ExoticsUnselected {
                commands.entity(weapon_icon.0).despawn();
            }
        }

        tracing::info!("Spawn selected exotic icon");
        let weapon_icon = WeaponIcon::new_selected_exotic_icon();
        let texture = asset_server.load(weapon_icon.icon.to_string());

        commands
            .spawn(weapon_icon_sprite_bundle(
                &weapon_icon,
                texture,
                Vec2::new(200.0, 100.0),
            ))
            .insert(weapon_icon);
    }
}

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
